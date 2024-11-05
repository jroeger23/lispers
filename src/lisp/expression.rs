use super::environment::{Environment, EnvironmentLayer};
use std::collections::HashMap;

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
fn eval(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
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
        Expression::Symbol(s) => eval(
            env,
            env.get(&s).ok_or(EvalError::SymbolNotBound(s)).cloned()?,
        ),
        x => Ok(x),
    }
}

//==================Prelude evaluation environment==============================

fn prelude_add(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;

    match eval(env, a)? {
        Expression::Integer(a) => match eval(env, b)? {
            Expression::Integer(b) => Ok(Expression::Integer(a + b)),
            Expression::Float(b) => Ok(Expression::Float(a as f64 + b)),
            x => Err(EvalError::NotANumber(x)),
        },
        Expression::Float(a) => match eval(env, b)? {
            Expression::Float(b) => Ok(Expression::Float(a + b)),
            Expression::Integer(b) => Ok(Expression::Float(a + b as f64)),
            x => Err(EvalError::NotANumber(x)),
        },
        x => Err(EvalError::NotANumber(x)),
    }
}

fn prelude_sub(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;

    match eval(env, a)? {
        Expression::Integer(a) => match eval(env, b)? {
            Expression::Integer(b) => Ok(Expression::Integer(a - b)),
            Expression::Float(b) => Ok(Expression::Float(a as f64 - b)),
            x => Err(EvalError::NotANumber(x)),
        },
        Expression::Float(a) => match eval(env, b)? {
            Expression::Float(b) => Ok(Expression::Float(a - b)),
            Expression::Integer(b) => Ok(Expression::Float(a - b as f64)),
            x => Err(EvalError::NotANumber(x)),
        },
        x => Err(EvalError::NotANumber(x)),
    }
}

fn prelude_mul(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;

    match eval(env, a)? {
        Expression::Integer(a) => match eval(env, b)? {
            Expression::Integer(b) => Ok(Expression::Integer(a * b)),
            Expression::Float(b) => Ok(Expression::Float(a as f64 * b)),
            x => Err(EvalError::NotANumber(x)),
        },
        Expression::Float(a) => match eval(env, b)? {
            Expression::Float(b) => Ok(Expression::Float(a * b)),
            Expression::Integer(b) => Ok(Expression::Float(a * b as f64)),
            x => Err(EvalError::NotANumber(x)),
        },
        x => Err(EvalError::NotANumber(x)),
    }
}

fn prelude_div(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;

    match eval(env, a)? {
        Expression::Integer(a) => match eval(env, b)? {
            Expression::Integer(b) => Ok(Expression::Integer(a / b)),
            Expression::Float(b) => Ok(Expression::Float(a as f64 / b)),
            x => Err(EvalError::NotANumber(x)),
        },
        Expression::Float(a) => match eval(env, b)? {
            Expression::Float(b) => Ok(Expression::Float(a / b)),
            Expression::Integer(b) => Ok(Expression::Float(a / b as f64)),
            x => Err(EvalError::NotANumber(x)),
        },
        x => Err(EvalError::NotANumber(x)),
    }
}

fn prelude_lambda(_env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [args, body] = expr.try_into()?;
    let mut arg_exprs: Vec<Expression> = args.try_into()?;
    let argument_symbols: Vec<String> = arg_exprs
        .iter_mut()
        .map(|a| match a {
            Expression::Symbol(s) => Ok(s.to_owned()),
            x => Err(EvalError::NotASymbol(x.to_owned())),
        })
        .collect::<Result<Vec<String>, EvalError>>()?;
    Ok(Expression::AnonymousFunction {
        argument_symbols,
        body: Box::new(body),
    })
}

fn prelude_let(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [bindings, body] = expr.try_into()?;

    let bindings = CellIterator::new(bindings)
        .map(|e| {
            let (s, e) = e?.try_into()?;
            if let Expression::Symbol(s) = s {
                Ok((s, eval(env, e)?))
            } else {
                Err(EvalError::ArgumentError(
                    "Let bindings must be an alist with elements (symbol . expr)".to_string(),
                ))
            }
        })
        .collect::<Result<HashMap<String, Expression>, EvalError>>()?;

    eval(&env.overlay(bindings.into()), body)
}

fn prelude_if(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [predicate, e_then, e_else] = expr.try_into()?;

    match eval(env, predicate)? {
        Expression::Nil => eval(env, e_else),
        _ => eval(env, e_then),
    }
}

fn prelude_eq(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;
    let a = eval(env, a)?;
    let b = eval(env, b)?;

    if a == b {
        Ok(Expression::True)
    } else {
        Ok(Expression::Nil)
    }
}

fn prelude_lt(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;
    let a = eval(env, a)?;
    let b = eval(env, b)?;

    if a < b {
        Ok(Expression::True)
    } else {
        Ok(Expression::Nil)
    }
}

fn prelude_gt(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;
    let a = eval(env, a)?;
    let b = eval(env, b)?;

    if a > b {
        Ok(Expression::True)
    } else {
        Ok(Expression::Nil)
    }
}

pub fn eval_prelude(expr: Expression) -> Result<Expression, EvalError> {
    let mut prelude = Environment::new();
    prelude.set("+".to_string(), Expression::Function(prelude_add));
    prelude.set("-".to_string(), Expression::Function(prelude_sub));
    prelude.set("*".to_string(), Expression::Function(prelude_mul));
    prelude.set("/".to_string(), Expression::Function(prelude_div));
    prelude.set("lambda".to_string(), Expression::Function(prelude_lambda));
    prelude.set("if".to_string(), Expression::Function(prelude_if));
    prelude.set("==".to_string(), Expression::Function(prelude_eq));
    prelude.set("<".to_string(), Expression::Function(prelude_lt));
    prelude.set(">".to_string(), Expression::Function(prelude_gt));
    prelude.set("let".to_string(), Expression::Function(prelude_let));

    eval(&prelude, expr)
}
