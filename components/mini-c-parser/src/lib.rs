//! The parser is written with LALRPOP.

// Export
pub use lalrpop_codegen::mini_c_grammar::*;
// wtf, why this here
pub use mini_c_lexer::MiniCParseError;

#[allow(warnings)]
mod lalrpop_codegen {
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub mini_c_grammar);
}

pub type LalrpopError<'a> =
    lalrpop_util::ParseError<usize, mini_c_lexer::Token<'a>, MiniCParseError>;

// Wrap a MiniCParseError in an Lalrpop error, used in the grammar file.
pub fn error(error: MiniCParseError) -> LalrpopError<'static> {
    LalrpopError::User { error }
}
