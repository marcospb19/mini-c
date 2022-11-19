use sushi_ast::{BinaryOperation, Expression, Value};
use sushi_error::RuntimeResult;

use crate::{environment::Environment, interpret::Evaluate};

impl Evaluate<Value> for Expression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        todo!()
        // let value: Value = match self {
        //     Self::Binary(lhs, op, rhs) => {
        //         let left = lhs.evaluate(environment)?;
        //         let right = rhs.evaluate(environment)?;

        //         match (left, op, right) {
        //             (Value::Bool(l), BinaryOperation::Equals, Value::Bool(r)) => {}
        //             _ => {}
        //         }

        //         todo!()
        //     }
        //     Self::Literal(value) => value.clone(),
        //     Self::Variable(ident) => todo!(),
        // };

        // Ok(value)
    }
}
