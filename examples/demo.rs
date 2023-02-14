use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{List, ListItem, ListState},
    Frame, Terminal,
};
use tui_elm::{run, Command, Message, Model};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run(&mut terminal, App::default()).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}

#[derive(Debug)]
pub enum AppMessage {
    SetListItems(Vec<String>),
}

#[derive(Default)]
pub struct App {
    list_items: Vec<String>,
    list_index: Option<usize>,
    list_state: ListState,
}

impl Model for App {
    type CustomMessage = AppMessage;

    fn init(&self) -> Option<Command<Message<Self::CustomMessage>>> {
        Some(Command::new_async(async move {
            Message::Custom(AppMessage::SetListItems(vec![
                "first item".to_owned(),
                "second_item".to_owned(),
            ]))
        }))
    }

    fn update(
        &mut self,
        msg: tui_elm::Message<Self::CustomMessage>,
    ) -> Option<Command<Message<Self::CustomMessage>>> {
        match msg {
            Message::Custom(AppMessage::SetListItems(items)) => {
                self.list_items = items;
                if self.list_items.is_empty() {
                    self.list_index = None;
                } else {
                    self.list_index = Some(0);
                }
            }
            Message::TermEvent(Event::Key(KeyEvent {
                code: KeyCode::Char('q' | 'Q'),
                ..
            })) => {
                return Some(Command::new_async(async move { Message::Quit }));
            }
            Message::TermEvent(Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            })) => {
                if let Some(list_index) = self.list_index.as_mut() {
                    if *list_index > 0 {
                        *list_index -= 1;
                        self.list_state.select(Some(*list_index));
                    }
                }
            }
            Message::TermEvent(Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            })) => {
                if let Some(list_index) = self.list_index.as_mut() {
                    if *list_index < self.list_items.len() - 1 {
                        *list_index += 1;
                        self.list_state.select(Some(*list_index));
                    }
                }
            }
            _ => {}
        }
        None
    }

    fn view(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
        terminal.draw(|f| ui(f, self)).unwrap();
    }
}
fn ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &App) {
    let items: Vec<ListItem> = app
        .list_items
        .iter()
        .map(|l| ListItem::new(l.clone()))
        .collect();
    f.render_stateful_widget(
        List::new(items).highlight_style(Style::default().fg(Color::Green).bg(Color::Black)),
        f.size(),
        &mut app.list_state.clone(),
    )
}
