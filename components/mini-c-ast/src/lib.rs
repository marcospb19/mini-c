pub type Ident = String;

pub struct Program {
    pub preamble: bool,
    pub declarations: Vec<Declaration>,
}

pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
}

pub struct VariableDeclaration(pub Type, pub Vec<VariableDeclarationItem>);

pub enum Type {
    Void,
    Int,
    Bool,
}

pub enum VariableDeclarationItem {
    Normal(Ident),
    Array(Ident, i64),
}

pub struct FunctionDeclaration(pub Type, pub Ident, pub Vec<Parameter>, pub Option<Scope>);

pub struct Parameter(pub Type, pub Ident);

pub struct Scope(pub Vec<VariableDeclaration>, pub Vec<Statement>);

pub enum Statement {
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
    Break,
    Return(ReturnStatement),
    Scope(Scope),
}
pub struct IfStatement(pub Expression, pub Scope, pub Option<Scope>);
pub struct ForStatement(
    pub (Option<Expression>, Option<Expression>, Option<Expression>),
    pub Scope,
);
pub struct WhileStatement(pub Option<Expression>, pub Scope);
pub struct ReturnStatement(pub Option<Expression>);

pub enum Expression {
    Variable(Ident),
    Value(Value),
    ArrayIndexing(Box<ArrayIndexingExpression>),
    Assignment(Box<AssignmentExpression>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    FunctionCall(FunctionCallExpression),
}
pub struct ArrayIndexingExpression(pub Ident, pub Expression);
pub struct AssignmentExpression(pub Ident, pub Expression);
pub struct BinaryExpression(pub Expression, pub BinaryOperator, pub Expression);
pub struct UnaryExpression(pub UnaryOperator, pub Expression);
pub struct FunctionCallExpression(pub Ident, pub Vec<Expression>);

pub enum Value {
    Int(i64),
    Bool(bool),
}

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

// // // // // // // // // ---

// #[derive(Debug, Clone)]
// pub enum Statement {
//     Empty,
//     Expression(Spanned<Expression>),
//     Let(Spanned<Ident>, Spanned<Expression>),
// }

// pub struct Function;

// #[derive(Debug, Clone)]
// pub enum Expression {
//     Unary(Spanned<UnaryOperator>, Box<Spanned<Expression>>),
//     Binary(
//         Box<Spanned<Expression>>,
//         Spanned<BinaryOperator>,
//         Box<Spanned<Expression>>,
//     ),
//     Literal(Spanned<Value>),
//     Variable(Spanned<Ident>),
//     FunctionCall(Spanned<Ident>, Vec<Spanned<Expression>>),
// }

// #[derive(Debug, Clone)]
// pub enum BinaryOperator {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
//     Equals,
//     NotEquals,
//     Less,
//     LessOrEquals,
//     Greater,
//     GreaterOrEquals,
// }

// #[derive(Debug, Clone)]
// pub enum UnaryOperator {
//     Not,
//     Minus,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Value {
//     Int(i32),
//     Bool(bool),
//     Unit,
// }

// impl Value {
//     pub fn value_type_str(&self) -> &'static str {
//         match self {
//             Self::Int(_) => "{int}",
//             Self::Bool(_) => "{bool}",
//             Self::Unit => "()",
//         }
//     }
// }

// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::Int(int) => write!(f, "{int}"),
//             Self::Bool(boolean) => write!(f, "{boolean}"),
//             Self::Unit => write!(f, "()"),
//         }
//     }
// }
