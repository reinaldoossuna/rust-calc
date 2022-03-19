use std::collections::VecDeque;

pub struct App {
    pub input: String,
    pub list: VecDeque<f32>,
}

impl App {
    pub fn read_input(&mut self) {
        let itens: Vec<CalculatorItem> = self
            .input
            .as_str()
            .trim() // remove white spaces in the end begin
            .split_whitespace()
            .map(|str| parse(str))
            .collect();
        // parse to Number or Operation
        self.input.clear();
        for item in itens {
            match item {
                CalculatorItem::Number(n) => self.list.push_back(n),
                CalculatorItem::Op(op) => self.do_math(op),
            }
        }
    }

    fn do_math(&mut self, op: Operation) {
        match (self.list.pop_back(), self.list.pop_back()) {
            (Some(n1), Some(n2)) => {
                let result = match op {
                    Operation::Sum => n1 + n2,
                    Operation::Minus => n1 - n2,
                    Operation::Mult => n1 * n2,
                    Operation::Div => n1 / n2,
                };
                self.list.push_back(result);
            }
            (Some(n1), None) => {
                self.list.push_back(n1);
            }
            (_, _) => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            list: VecDeque::new(),
        }
    }
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
