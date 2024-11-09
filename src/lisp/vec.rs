use std::fmt::Display;

use as_any::AsAny;

use super::{
    environment::{Environment, EnvironmentLayer},
    eval::{eval, EvalError},
    expression::ForeignData,
    expression::{Expression, ForeignDataWrapper},
};

#[derive(Debug, Clone, Copy)]
/// A simple 3d vector.
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(vec3 {} {} {})", self.x, self.y, self.z)
    }
}

impl ForeignData for Vec3 {
    fn clone_data(&self) -> Box<dyn ForeignData> {
        Box::new(*self)
    }
    fn eq(&self, other: &dyn ForeignData) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Vec3>() {
            self.x == other.x && self.y == other.y && self.z == other.z
        } else {
            false
        }
    }
    fn partial_cmp(&self, other: &dyn ForeignData) -> Option<std::cmp::Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<Vec3>() {
            Some(
                self.x
                    .partial_cmp(&other.x)?
                    .then(self.y.partial_cmp(&other.y)?)
                    .then(self.z.partial_cmp(&other.z)?),
            )
        } else {
            None
        }
    }
}

impl TryFrom<Expression> for Vec3 {
    type Error = EvalError;
    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::ForeignExpression(fe) => {
                if let Some(vec) = fe.data.as_ref().as_any().downcast_ref::<Vec3>() {
                    Ok(*vec)
                } else {
                    Err(EvalError::TypeError("Expected vec3".to_string()))
                }
            }
            _ => Err(EvalError::TypeError("Expected vec3".to_string())),
        }
    }
}

/// Create a vec3 expression from a list of 3 floats
pub fn vec_vec(_env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [x, y, z]: [f64; 3] = expr.try_into()?;

    Ok(Expression::ForeignExpression(ForeignDataWrapper::new(
        Box::new(Vec3 { x, y, z }),
    )))
}

/// Add two vec3 expressions
pub fn vec_add(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [a, b]: [Expression; 2] = expr.try_into()?;

    let a = Vec3::try_from(eval(env, a)?)?;
    let b = Vec3::try_from(eval(env, b)?)?;

    Ok(Expression::ForeignExpression(ForeignDataWrapper::new(
        Box::new(Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }),
    )))
}

/// Add vec3 functions to a layer
pub fn mk_vec3(layer: &mut EnvironmentLayer) {
    layer.set("vec3".to_string(), Expression::Function(vec_vec));
    layer.set("vec3-add".to_string(), Expression::Function(vec_add));
}
