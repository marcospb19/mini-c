pub type Ident = String;

pub enum Statement {
    Empty,
    Expression(Expression),
}

pub enum Expression {
    Binary(Box<Expression>, BinaryOperation, Box<Expression>),
    Literal(Value),
    Variable(Ident),
}

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

pub enum Value {
    Number(i32),
    Bool(bool),
    Unit,
}
