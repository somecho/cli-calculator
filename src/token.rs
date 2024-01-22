#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    OpenParen,
    CloseParen,
    Floor,
    Ceil,
    Max,
    Min,
    Sqrt,
    Pow,
    Cos,
    Sin,
    Tan,
    Log,
    Comma,
    Number(f32),
}

impl Token {
    pub fn get_number(&self) -> Result<f32, String> {
        match self {
            Self::Number(n) => Ok(*n),
            _ => Err(format!("{} is not a number token", self)),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Plus => String::from("+"),
            Token::Minus => String::from("-"),
            Token::Star => String::from("*"),
            Token::Slash => String::from("/"),
            Token::Percent => String::from("%"),
            Token::OpenParen => String::from("("),
            Token::CloseParen => String::from(")"),
            Token::Max => String::from("max"),
            Token::Min => String::from("min"),
            Token::Sqrt => String::from("sqrt"),
            Token::Pow => String::from("pow"),
            Token::Comma => String::from(","),
            Token::Number(n) => n.to_string(),
            Token::Cos => String::from("cos"),
            Token::Sin => String::from("sin"),
            Token::Tan => String::from("tan"),
            Token::Log => String::from("log"),
            Token::Floor => String::from("floor"),
            Token::Ceil => String::from("ceil"),
        };
        write!(f, "{}", s)
    }
}
