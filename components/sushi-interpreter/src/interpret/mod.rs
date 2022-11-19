use sushi_error::RuntimeResult;

use crate::environment::Environment;

mod expression;
mod statement;

pub trait Evaluate<T = ()> {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<T>;
}
