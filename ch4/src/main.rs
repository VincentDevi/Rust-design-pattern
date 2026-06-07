enum Token {
    Number(f64),
    ResultReference(usize),
    Operator(char),
}

struct Calculator {
    current_value: f64,
    memory: Vec<f64>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            current_value: 0.0,
            memory: Vec::new(),
        }
    }

    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        if expression.starts_with("result") {
            if let Some(index) = expression.strip_prefix("result") {
                if let Ok(offset) = index.trim().parse::<usize>() {
                    return self.get_previous_result(offset);
                }
            }
        }
        let result = self.parse_and_evaluate(expression)?;
        self.memory.push(result);
        self.current_value = result;
        Ok(result)
    }

    fn get_previous_result(&self, index: usize) -> Result<f64, String> {
        if index == 0 {
            Ok(self.current_value)
        } else {
            let pos = self
                .memory
                .len()
                .checked_sub(index)
                .ok_or("Invalid result index")?;
            self.memory
                .get(pos)
                .copied()
                .ok_or_else(|| "Invalid result index".to_string())
        }
    }

    fn parse_and_evaluate(&mut self, expression: &str) -> Result<f64, String> {
        let tokens = self.tokenize(expression)?;
        for token in tokens {
            if let Token::ResultReference(index) = token {
                let _prev = self.get_previous_result(index)?;
            }
        }
        Ok(42.0)
    }

    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        for part in expression.split_whitespace() {
            let token = if let Some(index) = part.strip_prefix("result") {
                if let Ok(offset) = index.trim().parse() {
                    Token::ResultReference(offset)
                } else {
                    return Err("Invalid Result reference".to_string());
                }
            } else if let Ok(num) = part.parse() {
                Token::Number(num)
            } else if part.len() == 1 && "+-*/".contains(part) {
                Token::Operator(part.chars().next().unwrap())
            } else {
                return Err(format!("Invalid token: {}", part));
            };
            tokens.push(token);
        }
        Ok(tokens)
    }
}

fn main() {
    let mut calc = Calculator::new();

    match calc.evaluate("5 + 7") {
        Ok(result) => println!("5 + 7 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calc.evaluate("result0 * 2") {
        Ok(result) => println!("result0 * 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calc.evaluate("result1 - result0") {
        Ok(result) => println!("result1 - result0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
