#![feature(custom_inner_attributes)]
#![feature(box_syntax)]

pub mod ast;
pub mod lexer;
pub mod parser;

use std::io::{stderr, stdout, Write};

use self::{lexer::Lexer, parser::ProgramParser};

#[allow(dead_code)]
struct Environment<W1, W2>
where
    W1: Write,
    W2: Write,
{
    stdout: W1,
    stderr: W2,
}

pub fn run_sushi(_input: &str) -> Result<(), ()> {
    let (stdout, stderr) = (stdout(), stderr());
    let _environment = Environment { stdout, stderr };

    Ok(())
}

pub fn test_sushi(input: impl AsRef<str>) -> Result<OutputCapture, ()> {
    let input = input.as_ref();
    let (mut stdout, mut stderr) = (vec![], vec![]);
    let _environment = Environment {
        stdout: &mut stdout,
        stderr: &mut stderr,
    };

    let lexer = Lexer::new(input);
    let _ast = ProgramParser::new()
        .parse(input, lexer)
        .unwrap_or_else(|err| panic!("{err:?}"));

    let vec_to_string = |vec| String::from_utf8(vec).expect("Sushi output must be valid utf8");

    let (stdout, stderr) = (vec_to_string(stdout), vec_to_string(stderr));

    Ok(OutputCapture { stdout, stderr })
}

#[derive(Debug)]
pub struct OutputCapture {
    pub stdout: String,
    pub stderr: String,
}
