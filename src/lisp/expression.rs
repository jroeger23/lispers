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

impl From<Vec<Expression>> for Expression {
    fn from(mut value: Vec<Expression>) -> Self {
        let mut current = Expression::Nil;

        for e in value.iter_mut().rev() {
            current = Expression::Cell(Box::new(e.to_owned()), Box::new(current));
        }

        current
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
