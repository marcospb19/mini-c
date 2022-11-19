use std::collections::HashMap;

use sushi_ast::Ident;

use crate::value::Value;

struct Machine {
    variables: HashMap<Ident, Value>,
}
