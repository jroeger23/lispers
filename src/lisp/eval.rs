use std::fmt::Display;

use super::environment::Environment;
use super::environment::EnvironmentLayer;
use super::expression::Expression;

#[derive(Debug)]
/// All possible evaluation errors
pub enum EvalError {
    SymbolNotBound(String),
    NotAFunction(Expression),
    NotANumber(Expression),
    ArgumentError(String),
    TypeError(String),
    NotASymbol(Expression),
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EvalError::SymbolNotBound(s) => write!(f, "Symbol {} is not bound", s),
            EvalError::NotAFunction(e) => write!(f, "Expression {} is not a function", e),
            EvalError::NotANumber(e) => write!(f, "Expression {} is not a number", e),
            EvalError::ArgumentError(s) => write!(f, "Argument error: {}", s),
            EvalError::TypeError(s) => write!(f, "Type error: {}", s),
            EvalError::NotASymbol(e) => write!(f, "Expression {} is not a symbol", e),
        }
    }
}

/// A CellIterator is a convenience struct to iterate a linked cons list.
/// The Iterator returns Ok(Expression) as long, as there are elements in the list.
/// Err(EvalError) is returned when the right side of a cons cell is not another cons cell or nil.
pub struct CellIterator {
    expr: Option<Expression>,
}

impl CellIterator {
    pub fn new(expr: Expression) -> CellIterator {
        CellIterator { expr: Some(expr) }
    }
}

impl Iterator for CellIterator {
    type Item = Result<Expression, EvalError>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(expr) = self.expr.take() {
            match expr {
                Expression::Cell(head, tail) => {
                    self.expr = Some(*tail);
                    return Some(Ok(*head));
                }
                Expression::Nil => {
                    return None;
                }
                _ => {
                    return Some(Err(EvalError::TypeError(
                        "Expected a cell or nil".to_string(),
                    )));
                }
            }
        } else {
            None
        }
    }
}

/// Dispatch an anonymous function call. Evaluates `body` in `env`, binding `args` to `argument_symbols`
fn dispatch_anonymous_function(
    env: &Environment,
    argument_symbols: Vec<String>,
    body: Expression,
    args: Expression,
) -> Result<Expression, EvalError> {
    let mut args: Vec<Expression> = args.try_into()?;

    let mut overlay = EnvironmentLayer::new();

    if args.len() != argument_symbols.len() {
        return Err(EvalError::ArgumentError(format!(
            "Exprected {} arguments, got {}",
            argument_symbols.len(),
            args.len()
        )));
    }

    for (arg, symbol) in args.iter_mut().zip(argument_symbols.iter()) {
        overlay.set(symbol.to_owned(), arg.to_owned());
    }

    eval(&env.overlay(overlay), body)
}

/// Evaluate an expression inside an environment
pub fn eval(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    match expr {
        Expression::Cell(lhs, rhs) => match eval(env, *lhs)? {
            Expression::Function(f) => f(env, *rhs),
            Expression::AnonymousFunction {
                argument_symbols,
                body,
            } => dispatch_anonymous_function(env, argument_symbols, *body, *rhs),
            a => Err(EvalError::NotAFunction(a)),
        },
        Expression::Quote(e) => Ok(*e),
        Expression::Symbol(s) => eval(env, env.get(&s).ok_or(EvalError::SymbolNotBound(s))?),
        x => Ok(x),
    }
}
