#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    OpenParen,
    CloseParen,
    Max,
    Min,
    Sqrt,
    Pow,
    Comma,
    Number(f32),
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
        };
        write!(f, "{}", s)
    }
}
