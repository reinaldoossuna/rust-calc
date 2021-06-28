use std::collections::VecDeque;

pub struct App {
    pub input: String,
    pub list: VecDeque<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            list: VecDeque::new(),
        }
    }
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
