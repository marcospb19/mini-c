#![feature(custom_inner_attributes)]

use std::io::{self, Write};

use sushi_common::SushiConfig;
use sushi_interpreter::{Interpreter, InterpreterMode};

pub fn run_sushi_repl() -> io::Result<()> {
    let (mut stdout, mut stderr) = (io::stdout(), io::stderr());

    let mut interpreter = Interpreter::new(&mut stdout, &mut stderr, SushiConfig::with_colors());

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line)?;

        if bytes == 0 {
            return Ok(());
        }

        // If errors appear, report them and keep the REPL running.
        match interpreter.execute_file(&line, InterpreterMode::Repl) {
            Ok(_) => {}
            Err(err) => eprintln!("{err}"),
        }
    }
}

pub mod test_utils {
    use std::fmt;

    use super::*;

    pub fn test_sushi(input: impl AsRef<str>) -> OutputCapture {
        let (mut stdout, mut stderr) = (vec![], vec![]);

        let mut interpreter =
            Interpreter::new(&mut stdout, &mut stderr, SushiConfig::without_colors());

        interpreter
            .execute_file(input.as_ref(), InterpreterMode::Normal)
            .unwrap();

        let vec_to_string = |vec| String::from_utf8(vec).expect("Sushi output must be valid utf8");

        let [stdout, stderr] = [stdout, stderr].map(vec_to_string);

        OutputCapture { stdout, stderr }
    }

    #[derive(Debug)]
    pub struct OutputCapture {
        pub stdout: String,
        pub stderr: String,
    }

    impl fmt::Display for OutputCapture {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let Self { stdout, stderr } = self;

            writeln!(f, "### stdout ###")?;
            if stdout.is_empty() {
                writeln!(f, "[EMPTY]")?;
            } else {
                write!(f, "{stdout}")?;
            }

            writeln!(f, "### stderr ###")?;
            if stderr.is_empty() {
                writeln!(f, "[EMPTY]")
            } else {
                write!(f, "{stderr}")
            }
        }
    }
}
