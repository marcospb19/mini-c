use sushi_ast::Statement;
use sushi_error::RuntimeResult;

use crate::{environment::Environment, interpret::Evaluate};

impl Evaluate for Statement {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult {
        todo!();

        // match self {
        //     Self::Empty => {}
        //     Self::Expression(expression) => {
        //         let _ignored_value = expression.evaluate(environment);
        //     }
        // }

        // Ok(())
    }
}
