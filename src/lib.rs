#![feature(custom_inner_attributes)]
#![feature(box_syntax)]
#![feature(iter_intersperse)]

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;

use std::{
    fmt,
    io::{stderr, stdout, Write},
};

use self::{error::report_lalrpop_error, lexer::Lexer, parser::ProgramParser};

pub fn run_sushi(input: &str) -> Result<(), ()> {
    let (stdout, stderr) = (stdout(), stderr());
    let config = SushiConfig::with_colors();
    let mut environment = Environment {
        stdout,
        stderr,
        config,
    };

    environment.execute_file(input);
    Ok(())
}

pub fn test_sushi(input: impl AsRef<str>) -> Result<OutputCapture, ()> {
    let (mut stdout, mut stderr) = (vec![], vec![]);
    let mut environment = Environment {
        stdout: &mut stdout,
        stderr: &mut stderr,
        config: SushiConfig::without_colors(),
    };

    environment.execute_file(input.as_ref());

    let vec_to_string = |vec| String::from_utf8(vec).expect("Sushi output must be valid utf8");
    let (stdout, stderr) = (vec_to_string(stdout), vec_to_string(stderr));

    Ok(OutputCapture { stdout, stderr })
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

struct Environment<W1, W2>
where
    W1: Write,
    W2: Write,
{
    #[allow(dead_code)]
    stdout: W1,
    stderr: W2,
    config: SushiConfig,
}

impl<W1, W2> Environment<W1, W2>
where
    W1: Write,
    W2: Write,
{
    fn execute_file(&mut self, input: &str) {
        let lexer = Lexer::new(input);
        let ast = ProgramParser::new().parse(input, lexer);

        let _ast = match ast {
            Ok(ast) => ast,
            Err(err) => {
                report_lalrpop_error(&mut self.stderr, err, input, &self.config).unwrap();
                return;
            }
        };
    }
}

#[derive(Debug)]
pub struct SushiConfig {
    is_color_enabled: bool,
}

impl SushiConfig {
    pub fn with_colors() -> Self {
        Self {
            is_color_enabled: true,
        }
    }
    pub fn without_colors() -> Self {
        Self {
            is_color_enabled: false,
        }
    }
}
