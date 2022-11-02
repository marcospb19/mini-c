#![feature(custom_inner_attributes)]

mod lexer;

use std::io::{stderr, stdout, Write};

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

    return Ok(());
}

pub fn test_sushi(_input: impl AsRef<str>) -> Result<OutputCapture, ()> {
    let (mut stdout, mut stderr) = (vec![], vec![]);
    let _environment = Environment {
        stdout: &mut stdout,
        stderr: &mut stderr,
    };

    let vec_to_string = |vec| String::from_utf8(vec).expect("Sushi output must be valid utf8");

    let (stdout, stderr) = (vec_to_string(stdout), vec_to_string(stderr));

    return Ok(OutputCapture { stdout, stderr });
}

#[derive(Debug)]
pub struct OutputCapture {
    pub stdout: String,
    pub stderr: String,
}
