use sushi_ast::Statement;
use sushi_error::RuntimeResult;

use crate::evaluate::{Evaluate, Interpreter};

impl Evaluate for Statement {
    fn evaluate(&self, interpreter: &mut Interpreter) -> RuntimeResult {
        match self {
            Self::Empty => {}
            Self::Expression(expression) => {
                let _value = expression.evaluate(interpreter);
            }
            Self::Let(ident, expression) => {
                let value = expression.evaluate(interpreter)?;
                interpreter.machine.set_variable(ident.inner.clone(), value);
            }
        }

        Ok(())
    }
}
