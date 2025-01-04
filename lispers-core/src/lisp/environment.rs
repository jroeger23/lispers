use super::{expression::Expression, prelude::mk_prelude};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(PartialEq, Clone, Debug)]
/// A Environment is a stack of `EnvironmentLayer`s. Each `EnvironmentLayer` is a mapping from
/// variable names to their values.
pub struct Environment<'a> {
    /// The current mapping.
    layer: EnvironmentLayer,
    /// The outer _fallback_ mapping.
    outer: Option<&'a Environment<'a>>,
    /// A shared layer taking precendence over the outer layer, but not the current layer.
    shared: Rc<RefCell<EnvironmentLayer>>,
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
    pub fn get(&self, key: &str) -> Option<Expression> {
        self.symbols.get(key).cloned()
    }
}

impl From<HashMap<String, Expression>> for EnvironmentLayer {
    fn from(map: HashMap<String, Expression>) -> Self {
        EnvironmentLayer { symbols: map }
    }
}

impl<'a> Environment<'a> {
    /// Construct an empty `Environment`.
    pub fn new() -> Self {
        Environment {
            layer: EnvironmentLayer::new(),
            outer: None,
            shared: Rc::new(RefCell::new(EnvironmentLayer::new())),
        }
    }

    /// Construct an `Environment` from a `EnvironmentLayer` with no outer `Environment`.
    pub fn from_layer(layer: EnvironmentLayer) -> Self {
        Environment {
            layer,
            outer: None,
            shared: Rc::new(RefCell::new(EnvironmentLayer::new())),
        }
    }

    /// Construct a new `Environment` with `self` as the outer `Environment`.
    pub fn mk_inner(&self) -> Environment {
        Environment {
            layer: EnvironmentLayer::new(),
            outer: Some(self),
            shared: self.shared.clone(),
        }
    }

    /// Construct a new `Environment` with `self` as the outer `Environment` and `layer` as the
    pub fn overlay(&'a self, layer: EnvironmentLayer) -> Environment<'a> {
        Environment {
            layer,
            outer: Some(&self),
            shared: self.shared.clone(),
        }
    }

    /// Set a value in the shared layer.
    ///
    /// Panics:
    /// - if the shared layer cannot be borrowed mutably.
    pub fn shared_set(&self, key: String, value: Expression) {
        match self.shared.try_borrow_mut() {
            Ok(mut shared) => shared.set(key, value),
            Err(e) => panic!("Cannot borrow shared layer mutably. ({})", e),
        }
    }

    /// Get a value from the shared layer.
    pub fn shared_get(&self, key: &str) -> Option<Expression> {
        self.shared.borrow().get(key)
    }

    /// Get a value from the `Environment`, without looking at the shared layer.
    pub fn layer_get(&self, key: &str) -> Option<Expression> {
        if let Some(e) = self.layer.get(key) {
            Some(e)
        } else {
            self.outer?.layer_get(key).clone()
        }
    }

    /// Get a value from the `Environment`.
    pub fn get(&self, key: &str) -> Option<Expression> {
        if let Some(e) = self.layer.get(key) {
            Some(e)
        } else if let Some(e) = self.shared_get(key) {
            Some(e)
        } else {
            self.outer?.layer_get(key).clone()
        }
    }

    /// Set a value in the current `EnvironmentLayer`.
    pub fn set(&mut self, key: String, value: Expression) {
        self.layer.set(key, value);
    }
}

impl Default for Environment<'_> {
    /// Get the default prelude layer
    fn default() -> Self {
        let mut d = EnvironmentLayer::new();
        mk_prelude(&mut d);
        Environment {
            layer: d,
            outer: None,
            shared: Rc::new(RefCell::new(EnvironmentLayer::new())),
        }
    }
}

#[test]
fn test_environment() {
    let mut env = Environment::new();
    env.set("a".to_string(), Expression::Integer(1));
    env.set("b".to_string(), Expression::Integer(2));
    let mut inner = env.mk_inner();
    inner.set("a".to_string(), Expression::Integer(3));
    assert_eq!(inner.get("a"), Some(Expression::Integer(3)));
    assert_eq!(inner.get("b"), Some(Expression::Integer(2)));
    assert_eq!(env.get("a"), Some(Expression::Integer(1)));
    assert_eq!(env.get("b"), Some(Expression::Integer(2)));
    assert_eq!(env.get("c"), None);
}
