#![feature(custom_inner_attributes)]

use std::io;

use sushi_common::SushiConfig;
use sushi_interpreter::environment::Environment;

pub fn run_sushi(input: &str) -> io::Result<()> {
    let (mut stdout, mut stderr) = (io::stdout(), io::stderr());

    let mut environment = Environment {
        stdout: &mut stdout,
        stderr: &mut stderr,
        config: SushiConfig::with_colors(),
    };

    environment.execute_file(input)?;
    Ok(())
}

pub mod test_utils {
    use std::fmt;

    use super::*;

    pub fn test_sushi(input: impl AsRef<str>) -> OutputCapture {
        let (mut stdout, mut stderr) = (vec![], vec![]);
        let mut environment = Environment {
            stdout: &mut stdout,
            stderr: &mut stderr,
            config: SushiConfig::without_colors(),
        };

        environment.execute_file(input.as_ref()).unwrap();

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
