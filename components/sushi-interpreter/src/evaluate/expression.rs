use sushi_ast::{BinaryOperator, Expression, Spanned, Value};
use sushi_error::RuntimeResult;

use crate::evaluate::{Evaluate, Interpreter};

impl Evaluate<Value> for Expression {
    fn evaluate(&self, interpreter: &mut Interpreter) -> RuntimeResult<Value> {
        match self {
            Self::Unary(operator, expression) => {
                let span = expression.span;
                let value = expression.evaluate(interpreter)?;
                let value = Spanned::new(value, span);

                unary_ops::run_unary_operation(operator, value)
            }
            Self::Binary(lhs, operator, rhs) => {
                let (left_span, right_span) = (lhs.span, rhs.span);

                let lhs = lhs.evaluate(interpreter)?;
                let rhs = rhs.evaluate(interpreter)?;

                let lhs = Spanned::new(lhs, left_span);
                let rhs = Spanned::new(rhs, right_span);

                binary_ops::run_binary_operation(operator, lhs, rhs)
            }
            Self::Literal(value) => Ok(value.inner.clone()),
            Self::Variable(ident) => Ok(interpreter.machine.access_variable(&ident.inner).unwrap()),
            Self::FunctionCall(ident, args) => {
                if interpreter.machine.is_builtin_function(&ident.inner) {
                    interpreter.call_builtin_function(ident, args)
                } else if interpreter.machine.is_user_function(&ident.inner) {
                    interpreter.call_user_function(ident, args)
                } else {
                    panic!("throw error here, called function is not defined.")
                }
            }
        }
    }
}

mod binary_ops {
    use sushi_ast::{Span, Spanned, Value::*};
    use sushi_error::binary_operator_error;

    use super::*;

    pub fn run_binary_operation(
        operator: &Spanned<BinaryOperator>,
        lhs: Spanned<Value>,
        rhs: Spanned<Value>,
    ) -> RuntimeResult<Value> {
        match operator.inner {
            BinaryOperator::Add => binary_ops::add(lhs, rhs, operator.span),
            BinaryOperator::Subtract => binary_ops::subtract(lhs, rhs, operator.span),
            BinaryOperator::Multiply => binary_ops::multiply(lhs, rhs, operator.span),
            BinaryOperator::Divide => binary_ops::divide(lhs, rhs, operator.span),
            BinaryOperator::Equals => binary_ops::equals(lhs, rhs, operator.span),
            BinaryOperator::NotEquals => binary_ops::not_equals(lhs, rhs, operator.span),
            BinaryOperator::Less => binary_ops::less(lhs, rhs, operator.span),
            BinaryOperator::LessOrEquals => binary_ops::less_or_equals(lhs, rhs, operator.span),
            BinaryOperator::Greater => binary_ops::greater(lhs, rhs, operator.span),
            BinaryOperator::GreaterOrEquals => {
                binary_ops::greater_or_equals(lhs, rhs, operator.span)
            }
        }
    }

    fn add(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Int(l + r))
    }

    fn subtract(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Int(l - r))
    }

    fn multiply(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Int(l * r))
    }

    fn divide(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Int(l / r))
    }

    fn equals(
        left: Spanned<Value>,
        right: Spanned<Value>,
        _operator_span: Span,
    ) -> RuntimeResult<Value> {
        Ok(Bool(left.inner == right.inner))
    }

    fn not_equals(
        left: Spanned<Value>,
        right: Spanned<Value>,
        _operator_span: Span,
    ) -> RuntimeResult<Value> {
        Ok(Bool(left.inner != right.inner))
    }

    fn less(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Bool(l < r))
    }

    fn less_or_equals(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Bool(l <= r))
    }

    fn greater(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Bool(l > r))
    }

    fn greater_or_equals(
        left: Spanned<Value>,
        right: Spanned<Value>,
        operator_span: Span,
    ) -> RuntimeResult<Value> {
        let (Int(l), Int(r)) = (&left.inner, &right.inner) else {
            return Err(binary_operator_error(left, right, operator_span));
        };

        Ok(Bool(l >= r))
    }
}

mod unary_ops {
    use sushi_ast::{Span, Spanned, UnaryOperator, Value::*};
    use sushi_error::unary_operator_error;

    use super::*;

    pub fn run_unary_operation(
        operator: &Spanned<UnaryOperator>,
        value: Spanned<Value>,
    ) -> RuntimeResult<Value> {
        match operator.inner {
            UnaryOperator::Not => unary_ops::not(operator.span, value),
            UnaryOperator::Minus => unary_ops::minus(operator.span, value),
        }
    }

    fn not(operator_span: Span, value: Spanned<Value>) -> RuntimeResult<Value> {
        let Bool(boolean) = &value.inner else {
            return Err(unary_operator_error(value, operator_span));
        };

        Ok(Bool(!boolean))
    }

    fn minus(operator_span: Span, value: Spanned<Value>) -> RuntimeResult<Value> {
        let Int(int) = &value.inner else {
            return Err(unary_operator_error(value, operator_span));
        };

        Ok(Int(-int))
    }
}
