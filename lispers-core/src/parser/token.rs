use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
/// Sum type of different tokens
pub enum Token {
    FloatLiteral(f64),
    IntLiteral(i64),
    Dot,
    Nil,
    ParClose,
    ParOpen,
    Quote,
    StringLiteral(String),
    Symbol(String),
    True,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::FloatLiteral(x) => write!(f, "{}", x),
            Token::IntLiteral(x) => write!(f, "{}", x),
            Token::Dot => write!(f, "."),
            Token::Nil => write!(f, "nil"),
            Token::ParClose => write!(f, ")"),
            Token::ParOpen => write!(f, "("),
            Token::Quote => write!(f, "'"),
            Token::StringLiteral(x) => write!(f, "\"{}\"", x),
            Token::Symbol(x) => write!(f, "{}", x),
            Token::True => write!(f, "true"),
        }
    }
}
