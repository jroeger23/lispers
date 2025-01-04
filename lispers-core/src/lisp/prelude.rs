use super::environment::Environment;
use super::environment::EnvironmentLayer;
use super::eval::eval;
use super::eval::CellIterator;
use super::eval::EvalError;
use super::expression::Expression;
use std::collections::HashMap;

pub fn prelude_add(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
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

pub fn prelude_sub(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
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

pub fn prelude_mul(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
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

pub fn prelude_div(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
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

pub fn prelude_lambda(_env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [args, body]: [Expression; 2] = expr.try_into()?;
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

pub fn prelude_defun(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [name, args, body]: [Expression; 3] = expr.try_into()?;
    let name = match name {
        Expression::Symbol(s) => s,
        x => return Err(EvalError::NotASymbol(x)),
    };
    let mut arg_exprs: Vec<Expression> = args.try_into()?;
    let argument_symbols: Vec<String> = arg_exprs
        .iter_mut()
        .map(|a| match a.to_owned() {
            Expression::Symbol(s) => Ok(s),
            x => Err(EvalError::NotASymbol(x)),
        })
        .collect::<Result<Vec<String>, EvalError>>()?;

    let f = Expression::AnonymousFunction {
        argument_symbols,
        body: Box::new(body),
    };
    env.shared_set(name, f.clone());
    Ok(f)
}

pub fn prelude_define(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [name, value] = expr.try_into()?;
    let name = match name {
        Expression::Symbol(s) => s,
        x => return Err(EvalError::NotASymbol(x)),
    };
    let value = eval(env, value)?;
    env.shared_set(name, value.clone());
    Ok(value)
}

pub fn prelude_let(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [bindings, body] = expr.try_into()?;

    let bindings = CellIterator::new(eval(env, bindings)?)
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

pub fn prelude_if(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [predicate, e_then, e_else] = expr.try_into()?;

    match eval(env, predicate)? {
        Expression::Nil => eval(env, e_else),
        _ => eval(env, e_then),
    }
}

pub fn prelude_eq(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;
    let a = eval(env, a)?;
    let b = eval(env, b)?;

    if a == b {
        Ok(Expression::True)
    } else {
        Ok(Expression::Nil)
    }
}

pub fn prelude_lt(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;
    let a = eval(env, a)?;
    let b = eval(env, b)?;

    if a < b {
        Ok(Expression::True)
    } else {
        Ok(Expression::Nil)
    }
}

pub fn prelude_gt(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;
    let a = eval(env, a)?;
    let b = eval(env, b)?;

    if a > b {
        Ok(Expression::True)
    } else {
        Ok(Expression::Nil)
    }
}

pub fn prelude_not(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a] = expr.try_into()?;
    match eval(env, a)? {
        Expression::Nil => Ok(Expression::True),
        _ => Ok(Expression::Nil),
    }
}

pub fn prelude_set(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [s, e] = expr.try_into()?;

    match eval(env, s)? {
        Expression::Symbol(s) => {
            let e = eval(env, e)?;
            env.shared_set(s, e.clone());
            Ok(e)
        }
        x => Err(EvalError::NotASymbol(x)),
    }
}

pub fn prelude_print(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [e] = expr.try_into()?;
    let e = eval(env, e)?;
    println!("Prelude: {}", e);
    Ok(e)
}

pub fn prelude_cons(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b] = expr.try_into()?;
    Ok(Expression::Cell(
        Box::new(eval(env, a)?),
        Box::new(eval(env, b)?),
    ))
}

pub fn prelude_car(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [arg] = expr.try_into()?;
    let (a, _) = eval(env, arg)?.try_into()?;
    Ok(a)
}

pub fn prelude_cdr(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [arg] = expr.try_into()?;
    let (_, b) = eval(env, arg)?.try_into()?;
    Ok(b)
}

pub fn prelude_eval(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [e] = expr.try_into()?;
    eval(env, eval(env, e)?)
}

pub fn prelude_progn(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let mut result = Expression::Nil;

    for e in CellIterator::new(expr) {
        result = eval(env, e?)?;
    }

    Ok(result)
}

pub fn mk_prelude(layer: &mut EnvironmentLayer) {
    layer.set("+".to_string(), Expression::Function(prelude_add));
    layer.set("-".to_string(), Expression::Function(prelude_sub));
    layer.set("*".to_string(), Expression::Function(prelude_mul));
    layer.set("/".to_string(), Expression::Function(prelude_div));
    layer.set("lambda".to_string(), Expression::Function(prelude_lambda));
    layer.set("defun".to_string(), Expression::Function(prelude_defun));
    layer.set("define".to_string(), Expression::Function(prelude_define));
    layer.set("if".to_string(), Expression::Function(prelude_if));
    layer.set("=".to_string(), Expression::Function(prelude_eq));
    layer.set("<".to_string(), Expression::Function(prelude_lt));
    layer.set(">".to_string(), Expression::Function(prelude_gt));
    layer.set("not".to_string(), Expression::Function(prelude_not));
    layer.set("let".to_string(), Expression::Function(prelude_let));
    layer.set("set".to_string(), Expression::Function(prelude_set));
    layer.set("print".to_string(), Expression::Function(prelude_print));
    layer.set("cons".to_string(), Expression::Function(prelude_cons));
    layer.set("car".to_string(), Expression::Function(prelude_car));
    layer.set("cdr".to_string(), Expression::Function(prelude_cdr));
    layer.set("eval".to_string(), Expression::Function(prelude_eval));
    layer.set("progn".to_string(), Expression::Function(prelude_progn));
}
