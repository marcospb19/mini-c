//! The parser is written with LALRPOP.

#[allow(warnings, unused)]
mod lalrpop_codegen {
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub sushi_grammar);
}

// Export
pub use lalrpop_codegen::sushi_grammar::ProgramParser;
