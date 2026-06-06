use std::{io::Write, ops::Deref, process::exit};

struct OperandStack(Vec<Box<dyn Operand>>);

impl OperandStack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push_operand(&mut self, operand: Box<dyn Operand>) {
        self.0.push(operand);
    }

    fn pop_operand(&mut self) -> Box<dyn Operand> {
        self.0.pop().unwrap()
    }
}

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
    stack: OperandStack,
}

impl Deref for AdditionOperator {
    type Target = OperandStack;
    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}

impl AdditionOperator {
    fn new() -> Self {
        Self {
            stack: OperandStack::new(),
        }
    }
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
        self.stack.0.push(operand);
    }
    fn pop_operand(&mut self) -> Box<dyn Operand> {
        self.stack.0.pop().unwrap()
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

trait OprationType {
    fn calculate(&self, left: f64, right: f64) -> f64;
    fn precedence(&self) -> u8;
}

struct Add;

impl OprationType for Add {
    fn calculate(&self, left: f64, right: f64) -> f64 {
        left + right
    }
    fn precedence(&self) -> u8 {
        1
    }
}

struct Multiply;
impl OprationType for Multiply {
    fn calculate(&self, left: f64, right: f64) -> f64 {
        left * right
    }
    fn precedence(&self) -> u8 {
        2
    }
}

struct Opreration<T: OprationType> {
    symbol: String,
    op_type: T,
}

impl<T: OprationType> Opreration<T> {
    fn new(symbol: String, op_type: T) -> Self {
        Self { symbol, op_type }
    }

    fn evaluate(&self, left: f64, right: f64) -> f64 {
        self.op_type.calculate(left, right)
    }
    fn get_precedence(&self) -> u8 {
        self.op_type.precedence()
    }
}

fn evaluate_expression(expression: &str) -> Result<String, String> {
    todo!()
}

fn main() {
    let add_op = Opreration::new("+".to_string(), Add);
    let mul_op = Opreration::new("*".to_string(), Multiply);

    println!("5 {} 3 = {}", add_op.symbol, add_op.evaluate(5.0, 3.0));
    println!("5 {} 3 = {}", mul_op.symbol, mul_op.evaluate(5.0, 3.0));
}
