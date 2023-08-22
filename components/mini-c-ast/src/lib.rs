use serde::Serialize;

pub type Ident = String;

#[derive(Debug, Clone, Serialize)]
pub struct Program {
    pub preamble: bool,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Declaration {
    Variable(Vec<VariableDeclaration>),
    Function(FunctionDeclaration),
}

#[derive(Debug, Clone, Serialize)]
pub struct VariableDeclaration {
    pub ident: Ident,
    pub typ: Type,
    pub array_len: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq)]
pub enum Type {
    Void,
    Int,
    Bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionDeclaration(pub Type, pub Ident, pub Vec<Parameter>, pub Scope);

impl FunctionDeclaration {
    pub fn ident(&self) -> &Ident {
        &self.1
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Parameter(pub Type, pub Ident);

#[derive(Debug, Clone, Serialize)]
pub struct Scope(pub Vec<VariableDeclaration>, pub Vec<Statement>);

#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
    Break,
    Return(ReturnStatement),
    Scope(Scope),
    Expression(Expression),
}
#[derive(Debug, Clone, Serialize)]
pub struct IfStatement(pub Expression, pub Scope, pub Option<Scope>);
#[derive(Debug, Clone, Serialize)]
pub struct ForStatement(
    pub (Option<Expression>, Option<Expression>, Option<Expression>),
    pub Scope,
);
#[derive(Debug, Clone, Serialize)]
pub struct WhileStatement(pub Option<Expression>, pub Scope);
#[derive(Debug, Clone, Serialize)]
pub struct ReturnStatement(pub Option<Expression>);

#[derive(Debug, Clone, Serialize)]
pub enum Expression {
    Value(Value),
    Assignment(Box<AssignmentExpression>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    FunctionCall(FunctionCallExpression),
    VariableReference(Box<VariableReferenceExpression>),
}
#[derive(Debug, Clone, Serialize)]
pub struct AssignmentExpression(pub VariableReferenceExpression, pub Expression);
#[derive(Debug, Clone, Serialize)]
pub struct BinaryExpression(pub Expression, pub BinaryOperator, pub Expression);
#[derive(Debug, Clone, Serialize)]
pub struct UnaryExpression(pub UnaryOperator, pub Expression);
#[derive(Debug, Clone, Serialize)]
pub struct FunctionCallExpression(pub Ident, pub Vec<Expression>);
#[derive(Debug, Clone, Serialize)]
pub enum VariableReferenceExpression {
    Normal(Ident),
    Array(Ident, Expression),
}

#[derive(Debug, Clone, Serialize)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Equals,
    NotEquals,
    Greater,
    GreaterOrEquals,
    Less,
    LessOrEquals,
}

#[derive(Debug, Clone, Serialize)]
pub enum UnaryOperator {
    Not,
    Negative,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub inner: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }

    pub fn map_inner<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned {
            inner: f(self.inner),
            span: self.span,
        }
    }

    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned {
            inner: &self.inner,
            span: self.span,
        }
    }
}

impl<T, E> Spanned<Result<T, E>> {
    pub fn transpose_result(self) -> Result<Spanned<T>, E> {
        let span = self.span;

        match self.inner {
            Ok(inner) => Ok(Spanned { inner, span }),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct Span(usize, usize);

impl Span {
    pub fn new(l: usize, r: usize) -> Self {
        Self(l, r)
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(Span(start, end): Span) -> std::ops::Range<usize> {
        start..end
    }
}
