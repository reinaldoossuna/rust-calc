mod app;
#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use app::App;
use std::error::Error;
use std::{collections::VecDeque, io};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

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
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let mut text = Text::from("--- Calc ---");
            text.patch_style(Style::default());
            let header = Paragraph::new(text);
            f.render_widget(header, chunks[0]);

            let list: VecDeque<ListItem> = app
                .list
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                    ListItem::new(content)
                })
                .collect();

            let list = List::new(list).block(Block::default().borders(Borders::NONE).title("List"));
            f.render_widget(list, chunks[1]);

            let input = Paragraph::new(app.input.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(input, chunks[2]);

            // make the cursor visible
            f.set_cursor(chunks[2].x + app.input.width() as u16 + 1, chunks[2].y + 1);
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('\n') => {
                    app.read_input();
                }
                Key::Char(c) if (c.is_numeric() || c.is_whitespace()) => {
                    app.input.push(c);
                }
                Key::Char(c) if (c == '+' || c == '-' || c == '*' || c == '/') => {
                    app.input.push(c);
                    app.read_input();
                }
                Key::Char(c) if c == 'd' => {
                    app.list.pop_back();
                }

                Key::Char(c) if c == 'q' => {
                    break;
                }
                Key::Backspace => {
                    app.input.pop();
                }
                Key::Ctrl('c') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
