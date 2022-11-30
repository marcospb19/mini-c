mod expression;
mod spanned;
mod statement;

use sushi_error::RuntimeResult;

use crate::Interpreter;

pub trait Evaluate<T = ()> {
    fn evaluate(&self, interpreter: &mut Interpreter) -> RuntimeResult<T>;
}
