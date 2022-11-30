use std::collections::HashMap;

use sushi_ast::{Function, Ident, Value};

#[derive(Default)]
pub struct Machine {
    variables: HashMap<Ident, Value>,
    functions: HashMap<Ident, Function>,
}

impl Machine {
    pub fn access_variable(&self, ident: &Ident) -> Option<Value> {
        self.variables.get(ident).cloned()
    }

    pub fn set_variable(&mut self, ident: Ident, value: Value) {
        self.variables.insert(ident, value);
    }

    pub fn is_builtin_function(&self, ident: &Ident) -> bool {
        matches!(ident.as_str(), "print" | "eprint" | "assert")
    }

    pub fn is_user_function(&self, ident: &Ident) -> bool {
        self.functions.contains_key(ident)
    }
}
