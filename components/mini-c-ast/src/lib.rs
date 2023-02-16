// use std::fmt;

pub type Ident = String;

pub struct Program {
    pub preamble: bool,
    pub declarations: Vec<Declaration>,
}

pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
}

pub struct VariableDeclaration(pub VarType, pub Vec<Ident>);

pub struct FunctionDeclaration(
    pub ReturnType,
    pub Ident,
    pub Vec<Parameter>,
    pub Option<Scope>,
);

pub struct Parameter(pub VarType, pub Ident);
pub type Scope = ();

pub enum VarType {
    Int,
    Bool,
}

pub enum ReturnType {
    Void,
    Int,
    Bool,
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
