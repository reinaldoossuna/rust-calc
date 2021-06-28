mod app;
#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use app::App;
use std::error::Error;
use std::io::Write;
use std::{collections::VecDeque, io};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
