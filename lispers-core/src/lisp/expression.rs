use std::any::Any;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;

use as_any::AsAny;

use super::environment::Environment;
use super::eval::CellIterator;
use super::eval::EvalError;

/// A trait for foreign data types that can be used in lisp expressions.
/// Note: This trait requires explicit implementation of:
/// - partial_cmp_impl
/// - clone_impl
/// - eq_impl
/// - as_any_box
/// to ensure object safety.
pub trait ForeignData: Debug + Display + AsAny {
    fn partial_cmp_impl(&self, other: &dyn ForeignData) -> Option<std::cmp::Ordering>;
    fn clone_impl(&self) -> Box<dyn ForeignData>;
    fn eq_impl(&self, other: &dyn ForeignData) -> bool;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Debug + Display + AsAny + PartialOrd + PartialEq + Clone + 'static> ForeignData for T {
    fn partial_cmp_impl(&self, other: &dyn ForeignData) -> Option<std::cmp::Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self.partial_cmp(other)
        } else {
            None
        }
    }

    fn clone_impl(&self) -> Box<dyn ForeignData> {
        Box::new(self.clone())
    }

    fn eq_impl(&self, other: &dyn ForeignData) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self.eq(other)
        } else {
            false
        }
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

/// A wrapper struct around any foreign data type. This struct is used to convert
/// any T implementing ForeignData to an Expression and vice versa.
#[derive(Debug)]
pub struct ForeignDataWrapper<T: ForeignData>(pub Box<T>);
impl<T: ForeignData> ForeignDataWrapper<T> {
    /// Create a new ForeignDataWrapper from an object implementing ForeignData.
    pub fn new(data: T) -> Self {
        ForeignDataWrapper(Box::new(data))
    }
}

impl<T: ForeignData> Deref for ForeignDataWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ForeignData> DerefMut for ForeignDataWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
/// A Store struct for foreign data types injected in expressions.
pub struct ForeignDataStore {
    /// The actual foreign data.
    data: Box<dyn ForeignData>,
}

/// The ForeignDataStore struct is used to store any foreign data type in an Expression
/// and cannot be constructed outside of this scope.
impl ForeignDataStore {
    /// Create a new ForeignDataStore from a ForeignData trait object.
    fn new(data: Box<dyn ForeignData>) -> Self {
        ForeignDataStore { data }
    }

    /// Get the contained box as an Any-Box with type info of the actual data.
    fn as_any_box(self) -> Box<dyn Any> {
        self.data.as_any_box()
    }
}

impl Clone for ForeignDataStore {
    fn clone(&self) -> Self {
        ForeignDataStore {
            data: self.data.clone_impl(),
        }
    }
}

impl PartialEq for ForeignDataStore {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq_impl(other.data.as_ref())
    }
}

impl PartialOrd for ForeignDataStore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp_impl(other.data.as_ref())
    }
}

impl Display for ForeignDataStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

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
    /// A foreign data expression.
    ForeignExpression(ForeignDataStore),
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

impl<T: ForeignData> From<ForeignDataWrapper<T>> for Expression {
    fn from(value: ForeignDataWrapper<T>) -> Expression {
        Expression::ForeignExpression(ForeignDataStore::new(value.0))
    }
}

impl<T: ForeignData> TryFrom<Expression> for ForeignDataWrapper<T> {
    type Error = EvalError;
    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::ForeignExpression(f) => match f.as_any_box().downcast::<T>() {
                Ok(data) => Ok(ForeignDataWrapper(data)),
                Err(_) => Err(EvalError::TypeError(
                    "Expression is not a ForeignDataWrapper".to_string(),
                )),
            },
            _ => Err(EvalError::TypeError(
                "Expression is not a ForeignDataWrapper".to_string(),
            )),
        }
    }
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

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Expression::Integer(value)
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Expression::Float(value)
    }
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Expression::String(value)
    }
}

impl From<bool> for Expression {
    fn from(value: bool) -> Self {
        if value {
            Expression::True
        } else {
            Expression::Nil
        }
    }
}

impl TryFrom<Expression> for i64 {
    type Error = EvalError;
    fn try_from(value: Expression) -> Result<i64, Self::Error> {
        match value {
            Expression::Integer(i) => Ok(i),
            _ => Err(EvalError::TypeError(
                "Expression is not an Integer".to_string(),
            )),
        }
    }
}

impl TryFrom<Expression> for f64 {
    type Error = EvalError;
    fn try_from(value: Expression) -> Result<f64, Self::Error> {
        match value {
            Expression::Integer(i) => Ok(i as f64),
            Expression::Float(f) => Ok(f),
            _ => Err(EvalError::TypeError(
                "Expression is not a Float".to_string(),
            )),
        }
    }
}

impl TryFrom<Expression> for String {
    type Error = EvalError;
    fn try_from(value: Expression) -> Result<String, Self::Error> {
        match value {
            Expression::String(s) => Ok(s),
            _ => Err(EvalError::TypeError(
                "Expression is not a String".to_string(),
            )),
        }
    }
}

impl TryFrom<Expression> for Vec<Expression> {
    type Error = EvalError;

    fn try_from(value: Expression) -> Result<Vec<Expression>, Self::Error> {
        CellIterator::new(value).collect()
    }
}

impl<ToExpr> TryFrom<Expression> for Vec<ToExpr>
where
    ToExpr: TryFrom<Expression, Error = EvalError>,
{
    type Error = EvalError;

    fn try_from(value: Expression) -> Result<Vec<ToExpr>, Self::Error> {
        CellIterator::new(value)
            .map(|x| x?.try_into() as Result<ToExpr, EvalError>)
            .collect()
    }
}

impl<ToExpr, const N: usize> TryFrom<Expression> for [ToExpr; N]
where
    ToExpr: TryFrom<Expression, Error = EvalError>,
{
    type Error = EvalError;

    fn try_from(value: Expression) -> Result<[ToExpr; N], Self::Error> {
        let buf: Vec<ToExpr> = value.try_into()?;
        let n = buf.len();

        buf.try_into()
            .map_err(|_| EvalError::ArgumentError(format!("Expected {} arguments, got {}", N, n)))
    }
}

impl<const N: usize> TryFrom<Expression> for [Expression; N] {
    type Error = EvalError;

    fn try_from(value: Expression) -> Result<[Expression; N], Self::Error> {
        let buf: Vec<Expression> = value.try_into()?;
        let n = buf.len();

        buf.try_into()
            .map_err(|_| EvalError::ArgumentError(format!("Expected {} arguments, got {}", N, n)))
    }
}

impl TryFrom<Expression> for (Expression, Expression) {
    type Error = EvalError;
    fn try_from(value: Expression) -> Result<(Expression, Expression), Self::Error> {
        match value {
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
            Expression::ForeignExpression(e) => write!(f, "{}", e),
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
            Expression::String(s) => write!(f, "{}", s),
            Expression::True => write!(f, "true"),
            Expression::Nil => write!(f, "nil"),
        }
    }
}
