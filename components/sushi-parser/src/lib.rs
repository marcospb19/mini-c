//! The parser is written with LALRPOP.

#![feature(box_syntax)]

#[allow(warnings)]
mod lalrpop_codegen {
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub sushi_grammar);
}

// Export
pub use lalrpop_codegen::sushi_grammar::{ExpressionParser, SushiParser};
