use std::fs;

use mini_c_lexer::Lexer;
use mini_c_llvm_codegen::{Compiler, LlvmContext};
use mini_c_parser::ProgramParser;

fn main() {
    let ctx: LlvmContext = LlvmContext::create();

    let examples = glob::glob("examples/*").unwrap();

    for path in examples {
        let path = path.unwrap();
        let input = fs::read_to_string(&path).unwrap();

        let lexer = Lexer::new(&input);
        let ast = ProgramParser::new().parse(&input, lexer).unwrap();

        let mut compiler = Compiler::new(&ctx);
        let compiled = compiler.compile(&ast);

        let _ = dbg!(compiled);
    }

    println!("all succeeded");
}
