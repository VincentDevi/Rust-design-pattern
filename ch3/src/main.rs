use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Variable(String),
    Operator(char),
}
struct ParsedExpression {
    tokens: Vec<Token>,
}

struct Calculation {
    expression: String,
    tokens: Vec<Token>,
    result: f64,
}
struct Calculator {
    variables: HashMap<String, f64>,
    history: Vec<Calculation>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }
    fn parse(&self, expr: &str) -> Result<ParsedExpression, String> {
        let tokens = self.tokenize(expr)?;
        Ok(ParsedExpression { tokens })
    }

    fn tokenize(&self, expr: &str) -> Result<Vec<Token>, String> {
        todo!()
    }

    fn evaluate_token(&self, tokens: Vec<Token>) -> Result<f64, String> {
        todo!()
    }

    fn evalutate_parse(&mut self, expr: String, parsed: ParsedExpression) -> Result<f64, String> {
        let result = self.evaluate_token(parsed.tokens.clone())?;
        self.history.push(Calculation {
            expression: expr,
            tokens: parsed.tokens,
            result,
        });
        Ok(result)
    }
    fn evaluate(&mut self, expr: String) -> Result<f64, String> {
        let parsed = self.parse(&expr)?;
        self.evalutate_parse(expr, parsed)
    }
}

fn main() {
    println!("Hello, world!");
}
