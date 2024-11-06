use std::fmt::Display;

use super::environment::Environment;
use super::eval::CellIterator;
use super::eval::EvalError;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// A sum type of all possible lisp expressions.
pub enum Expression {
    /// The classic lisp cons cell aka (a . b) used to construct expressions.
    Cell(Box<Expression>, Box<Expression>),
    /// A function expression pointing to native code.
    Function(fn(&Environment, Expression) -> Result<Expression, EvalError>),
    /// A anonymous function expression consisting of bound symbols and a body expression.
    AnonymousFunction {
        argument_symbols: Vec<String>,
        body: Box<Expression>,
    },
    /// A Quoted expression.
    Quote(Box<Expression>),
    /// A symbol.
    Symbol(String),
    /// Integer values.
    Integer(i64),
    /// Float values.
    Float(f64),
    /// String values.
    String(String),
    /// True
    True,
    /// Nil
    Nil,
}

impl From<fn(&Environment, Expression) -> Result<Expression, EvalError>> for Expression {
    fn from(f: fn(&Environment, Expression) -> Result<Expression, EvalError>) -> Self {
        Expression::Function(f)
    }
}

impl From<Vec<Expression>> for Expression {
    fn from(mut value: Vec<Expression>) -> Self {
        let mut current = Expression::Nil;

        for e in value.iter_mut().rev() {
            current = Expression::Cell(Box::new(e.to_owned()), Box::new(current));
        }

        current
    }
}

impl From<(Expression, Expression)> for Expression {
    fn from(value: (Expression, Expression)) -> Self {
        Expression::Cell(Box::new(value.0), Box::new(value.1))
    }
}

impl TryInto<i64> for Expression {
    type Error = EvalError;
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Expression::Integer(i) => Ok(i),
            _ => Err(EvalError::TypeError(
                "Expression is not an Integer".to_string(),
            )),
        }
    }
}

impl TryInto<f64> for Expression {
    type Error = EvalError;
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Expression::Float(f) => Ok(f),
            _ => Err(EvalError::TypeError(
                "Expression is not a Float".to_string(),
            )),
        }
    }
}

impl TryInto<String> for Expression {
    type Error = EvalError;
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Expression::String(s) => Ok(s),
            _ => Err(EvalError::TypeError(
                "Expression is not a String".to_string(),
            )),
        }
    }
}

impl TryInto<Vec<Expression>> for Expression {
    type Error = EvalError;

    fn try_into(self) -> Result<Vec<Expression>, Self::Error> {
        CellIterator::new(self).collect()
    }
}

impl<const N: usize> TryInto<[Expression; N]> for Expression {
    type Error = EvalError;

    fn try_into(self) -> Result<[Expression; N], Self::Error> {
        let buf: Vec<Expression> = self.try_into()?;
        let n = buf.len();

        buf.try_into()
            .map_err(|_| EvalError::ArgumentError(format!("Expected {} arguments, got {}", N, n)))
    }
}

impl TryInto<(Expression, Expression)> for Expression {
    type Error = EvalError;
    fn try_into(self) -> Result<(Expression, Expression), Self::Error> {
        match self {
            Expression::Cell(a, b) => Ok((*a, *b)),
            _ => Err(EvalError::TypeError(
                "Expression must be a Cell".to_string(),
            )),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Cell(a, b) => {
                match self.clone().try_into() as Result<Vec<Expression>, EvalError> {
                    Ok(lst) => write!(
                        f,
                        "({})",
                        lst.iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                    Err(_) => write!(f, "({} . {})", a, b),
                }
            }
            Expression::Function(_) => write!(f, "<function>"),
            Expression::AnonymousFunction {
                argument_symbols,
                body,
            } => write!(f, "(lambda ({}) {})", argument_symbols.join(" "), body),
            Expression::Quote(e) => write!(f, "'{}", e),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Float(fl) => write!(f, "{}", fl),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::True => write!(f, "true"),
            Expression::Nil => write!(f, "nil"),
        }
    }
}
