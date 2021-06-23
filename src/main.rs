#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::error::Error;
use std::io::Write;
use std::{collections::VecDeque, io};
use termion::{event::Key, raw::IntoRawMode};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

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

#[allow(dead_code)]
fn calc_loop() {
    let mut queue: VecDeque<f32> = VecDeque::new();

    print!("> ");
    io::stdout().flush().unwrap();

    loop {
        let expr = read_expr();
        for item in expr
            .trim() // remove white spaces in the end begin
            .split_whitespace()
            .map(|str| parse(str))
        // parse to Number or Operation
        {
            match item {
                CalculatorItem::Number(n) => queue.push_back(n),
                CalculatorItem::Op(op) => do_math(&mut queue, op),
            }
        }
    }
}

#[allow(dead_code)]
fn read_expr() -> String {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("failed to read input");

    expr
}

enum Operation {
    Sum,
    Minus,
    Mult,
    Div,
}

enum CalculatorItem {
    Number(f32),
    Op(Operation),
}

fn parse<'a>(expr: &'a str) -> CalculatorItem {
    match expr {
        "+" => CalculatorItem::Op(Operation::Sum),
        "-" => CalculatorItem::Op(Operation::Minus),
        "*" => CalculatorItem::Op(Operation::Mult),
        "/" => CalculatorItem::Op(Operation::Div),
        _ => {
            let num = expr.parse::<f32>().expect("not able to parse input");
            CalculatorItem::Number(num)
        }
    }
}

fn do_math(q: &mut VecDeque<f32>, op: Operation) {
    assert!(q.len() > 1);
    let n1 = q.pop_front().unwrap();
    let n2 = q.pop_front().unwrap();

    let result = match op {
        Operation::Sum => n1 + n2,
        Operation::Minus => n1 - n2,
        Operation::Mult => n1 * n2,
        Operation::Div => n1 / n2,
    };
    println!("= {}", result);
    q.push_back(result);
}
