use sushi_ast::Spanned;
use sushi_error::RuntimeResult;

use crate::evaluate::{Evaluate, Interpreter};

impl<T, U> Evaluate<T> for Spanned<U>
where
    U: Evaluate<T>,
{
    fn evaluate(&self, interpreter: &mut Interpreter) -> RuntimeResult<T> {
        let Spanned { span, inner } = self;
        interpreter.update_span(*span);
        inner.evaluate(interpreter)
    }
}
