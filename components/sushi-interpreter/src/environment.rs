use std::io;

use sushi_common::SushiConfig;
use sushi_error::{report_lalrpop_error, LalrpopError, RuntimeError};
use sushi_lexer::Lexer;
use sushi_parser::SushiParser;

use crate::interpret::Evaluate;

pub struct Environment<'a> {
    #[allow(dead_code)]
    pub stdout: &'a mut dyn io::Write,
    pub stderr: &'a mut dyn io::Write,
    pub config: SushiConfig,
}

impl Environment<'_> {
    pub fn execute_file(&mut self, input: &str) -> io::Result<()> {
        let lexer = Lexer::new(input);
        let ast = SushiParser::new().parse(input, lexer);

        let ast = match ast {
            Ok(ast) => ast,
            Err(err) => return self.report_lalrpop_error(err, input).map_err(From::from),
        };

        for statement in ast {
            if let Err(err) = statement.evaluate(self) {
                return self.report_runtime_error(err).map_err(From::from);
            }
        }

        Ok(())
    }

    fn report_lalrpop_error(&mut self, err: LalrpopError<'_>, input: &str) -> io::Result<()> {
        report_lalrpop_error(&mut self.stderr, err, input, &self.config)
    }

    fn report_runtime_error(&mut self, _err: RuntimeError) -> io::Result<()> {
        todo!()
    }
}
