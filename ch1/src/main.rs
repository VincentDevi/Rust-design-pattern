use std::{io::Write, process::exit};

trait Operand {
    fn evaluate(&self) -> f64;
}

trait Operator {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> char;
    fn push_operand(&mut self, operand: Box<dyn Operand>);
    fn pop_operand(&mut self) -> Box<dyn Operand>;
    fn apply(&mut self) -> Box<dyn Operand>;
}

trait UnaryOperator: Operator {
    fn apply_unary(&self, operand: Box<dyn Operand>) -> Box<dyn Operand>;
    fn apply(&mut self) -> Box<dyn Operand> {
        let operand = self.pop_operand();
        self.apply_unary(operand)
    }
}

trait BinaryOperator: Operator {
    fn apply_binary(
        &self,
        operand1: Box<dyn Operand>,
        operand2: Box<dyn Operand>,
    ) -> Box<dyn Operand>;

    fn apply(&mut self) -> Box<dyn Operand> {
        let operand2 = self.pop_operand();
        let operand1 = self.pop_operand();
        self.apply_binary(operand1, operand2)
    }
}

struct AdditionOperator {
    stack: Vec<Box<dyn Operand>>,
}

impl BinaryOperator for AdditionOperator {
    fn apply_binary(
        &self,
        operand1: Box<dyn Operand>,
        operand2: Box<dyn Operand>,
    ) -> Box<dyn Operand> {
        let inner_operand1 = operand1.evaluate();
        let inner_operand2 = operand2.evaluate();
        let result = inner_operand1 + inner_operand2;
        Box::new(Value(result))
    }
}

impl Operator for AdditionOperator {
    fn precedence(&self) -> u8 {
        0
    }
    fn symbol(&self) -> char {
        '+'
    }
    fn push_operand(&mut self, operand: Box<dyn Operand>) {
        self.stack.push(operand);
    }
    fn pop_operand(&mut self) -> Box<dyn Operand> {
        self.stack.pop().unwrap()
    }
    fn apply(&mut self) -> Box<dyn Operand> {
        let operand2 = self.pop_operand();
        let operand1 = self.pop_operand();
        self.apply_binary(operand1, operand2)
    }
}

struct Value(f64);

impl Operand for Value {
    fn evaluate(&self) -> f64 {
        self.0
    }
}

fn evaluate_expression(expression: &str) -> Result<String, String> {
    todo!()
}

fn main() {
    let mut buf = String::new();
    loop {
        print!(">");
        std::io::stdout().flush().unwrap();

        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();

        if buf.trim() == "exit" {
            exit(0)
        }

        match evaluate_expression(&buf) {
            Ok(result) => println!("{result}"),
            Err(error) => println!("Error: {error}"),
        }
    }
}
