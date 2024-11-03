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
