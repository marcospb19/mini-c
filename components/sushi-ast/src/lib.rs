pub type Ident = String;

#[derive(Debug, Clone)]
pub enum Statement {
    Empty,
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Box<Expression>, BinaryOperation, Box<Expression>),
    Literal(Value),
    Variable(Ident),
}

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    Less,
    LessOrEquals,
    Greater,
    GreaterOrEquals,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Bool(bool),
    Unit,
}
