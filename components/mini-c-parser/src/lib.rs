//! The parser is written with LALRPOP.

#![feature(box_syntax)]

#[allow(warnings)]
mod lalrpop_codegen {
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub mini_c_grammar);
}

// Export
// pub use lalrpop_codegen::mini_c_grammar::{ExpressionParser, SushiParser};

pub enum MiniCParseError {
    InvalidReturnType(String),
    InvalidVarType(String),
}

pub type LalrpopError<'a> =
    lalrpop_util::ParseError<usize, mini_c_lexer::Token<'a>, MiniCParseError>;

// Wrap a MiniCParseError in an Lalrpop error, used in the grammar file.
pub fn error<T>(error: MiniCParseError) -> Result<T, LalrpopError<'static>> {
    Err(LalrpopError::User { error })
}
