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
