#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Operator(Operator),
    Function(Function),
    Variable(String),
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
}

impl Token {
    pub fn number(value: f64) -> Self {
        Self::Number(value)
    }

    pub fn operator(op: Operator) -> Self {
        Self::Operator(op)
    }

    pub fn function(func: Function) -> Self {
        Self::Function(func)
    }

    pub fn variable(var: impl Into<String>) -> Self {
        Self::Variable(var.into())
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        if let Ok(num) = s.parse::<f64>() {
            return Ok(Self::Number(num));
        }
        match s {
            "+" => Ok(Self::operator(Operator::Add)),
            "-" => Ok(Self::operator(Operator::Substract)),
            "*" => Ok(Self::operator(Operator::Multiply)),
            "/" => Ok(Self::operator(Operator::Divide)),
            "sin" => Ok(Self::function(Function::Sin)),
            "cos" => Ok(Self::function(Function::Cos)),
            "sqrt" => Ok(Self::function(Function::Sqrt)),
            name if name.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                Ok(Self::variable(name))
            }
            _ => Err(format!("Invalid token : {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Function {
    Sqrt,
    Sin,
    Cos,
    Tan,
}

fn main() {
    let expr = vec![
        Token::number(2.0),
        Token::operator(Operator::Add),
        Token::number(3.0),
    ];

    let tokens: Result<Vec<Token>, _> = "2 + 3".split_whitespace().map(Token::from_str).collect();

    match tokens {
        Ok(expr) => println!("Valid expression {:?}", expr),
        Err(err) => println!("Error: {err}"),
    }
}
