use super::expression::Expression;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
/// A Environment is a stack of `EnvironmentLayer`s. Each `EnvironmentLayer` is a mapping from
/// variable names to their values.
pub struct Environment<'a> {
    /// The current mapping.
    layer: EnvironmentLayer,
    /// The outer _fallback_ mapping.
    outer: Option<&'a Environment<'a>>,
}

#[derive(PartialEq, Clone, Debug)]
/// A concrete EnvironmentLayer, containing a mapping from symbol names to Expressions.
pub struct EnvironmentLayer {
    symbols: HashMap<String, Expression>,
}

impl EnvironmentLayer {
    /// Construct an empty `EnvironmentLayer`.
    pub fn new() -> Self {
        EnvironmentLayer {
            symbols: HashMap::new(),
        }
    }

    /// Set a value in the `EnvironmentLayer`.
    pub fn set(&mut self, key: String, value: Expression) {
        self.symbols.insert(key, value);
    }

    /// Get a value in the `EnvironmentLayer`.
    pub fn get(&self, key: &str) -> Option<&Expression> {
        self.symbols.get(key)
    }
}

impl<'a> Environment<'a> {
    /// Construct an empty `Environment`.
    pub fn new() -> Self {
        Environment {
            layer: EnvironmentLayer::new(),
            outer: None,
        }
    }

    /// Construct an `Environment` from a `EnvironmentLayer` with no outer `Environment`.
    pub fn from_layer(layer: EnvironmentLayer) -> Self {
        Environment { layer, outer: None }
    }

    /// Construct a new `Environment` with `self` as the outer `Environment`.
    pub fn mk_inner(&self) -> Environment {
        Environment {
            layer: EnvironmentLayer::new(),
            outer: Some(self),
        }
    }

    /// Construct a new `Environment` with `self` as the outer `Environment` and `layer` as the
    pub fn overlay(&'a self, layer: EnvironmentLayer) -> Environment<'a> {
        Environment {
            layer,
            outer: Some(&self),
        }
    }

    /// Get a value from the `Environment`.
    pub fn get(&self, key: &str) -> Option<&Expression> {
        if let Some(e) = self.layer.get(key) {
            Some(e)
        } else {
            self.outer?.get(key)
        }
    }

    /// Set a value in the current `EnvironmentLayer`.
    pub fn set(&mut self, key: String, value: Expression) {
        self.layer.set(key, value);
    }
}

#[test]
fn test_environment() {
    let mut env = Environment::new();
    env.set("a".to_string(), Expression::Integer(1));
    env.set("b".to_string(), Expression::Integer(2));
    let mut inner = env.mk_inner();
    inner.set("a".to_string(), Expression::Integer(3));
    assert_eq!(inner.get("a"), Some(&Expression::Integer(3)));
    assert_eq!(inner.get("b"), Some(&Expression::Integer(2)));
    assert_eq!(env.get("a"), Some(&Expression::Integer(1)));
    assert_eq!(env.get("b"), Some(&Expression::Integer(2)));
    assert_eq!(env.get("c"), None);
}
