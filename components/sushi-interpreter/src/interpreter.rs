use std::{fmt::Write, io};

use sushi_ast::{Expression, Ident, Span, Spanned, Value};
use sushi_common::SushiConfig;
use sushi_error::{report_lalrpop_error, write_report, ErrorReport, LalrpopError, RuntimeResult};
use sushi_lexer::Lexer;
use sushi_parser::{ExpressionParser, SushiParser};

use crate::{evaluate::Evaluate, machine::Machine};

pub enum InterpreterMode {
    Repl,
    Normal,
}

pub struct Interpreter<'a> {
    current_span: Span,
    stdout: &'a mut dyn io::Write,
    stderr: &'a mut dyn io::Write,
    config: SushiConfig,
    pub(crate) machine: Machine,
}

impl<'a> Interpreter<'a> {
    pub fn new(
        stdout: &'a mut dyn io::Write,
        stderr: &'a mut dyn io::Write,
        config: SushiConfig,
    ) -> Self {
        Self {
            stdout,
            stderr,
            config,
            current_span: Span::default(),
            machine: Machine::default(),
        }
    }

    pub fn update_span(&mut self, span: Span) {
        self.current_span = span;
    }

    pub fn execute_file(&mut self, input: &str, mode: InterpreterMode) -> io::Result<()> {
        let lexer = Lexer::new(input);

        if let InterpreterMode::Repl = mode {
            let ast = ExpressionParser::new().parse(input, lexer);

            let expression = match ast {
                Ok(expression) => expression,
                Err(err) => return self.report_lalrpop_error(err, input).map_err(From::from),
            };

            let value = match expression.evaluate(self) {
                Ok(inner) => inner,
                Err(err) => return self.report_runtime_error(err, input).map_err(From::from),
            };

            return writeln!(self.stderr, "{value}");
        }

        let ast = SushiParser::new().parse(input, lexer);

        let ast = match ast {
            Ok(ast) => ast,
            Err(err) => return self.report_lalrpop_error(err, input).map_err(From::from),
        };

        for statement in ast {
            if let Err(err) = statement.evaluate(self) {
                return self.report_runtime_error(err, input).map_err(From::from);
            }
        }

        Ok(())
    }

    pub fn call_user_function(
        &mut self,
        _ident: &Spanned<Ident>,
        _args: &[Spanned<Expression>],
    ) -> RuntimeResult<Value> {
        todo!()
    }

    pub fn call_builtin_function(
        &mut self,
        ident: &Spanned<Ident>,
        args: &[Spanned<Expression>],
    ) -> RuntimeResult<Value> {
        match ident.inner.as_str() {
            ident @ ("print" | "eprint") => {
                let mut string = String::new();

                for (i, expression) in args.iter().enumerate() {
                    let value = expression.evaluate(self)?;

                    if i > 0 {
                        write!(string, ", ").unwrap();
                    }

                    write!(string, "{value}").unwrap();
                }

                let output = if ident.starts_with('e') {
                    &mut self.stderr
                } else {
                    &mut self.stdout
                };

                writeln!(output, "{string}").unwrap();
            }
            "assert" => {
                for expression in args {
                    dbg!(expression);
                    let value = expression.evaluate(self)?;
                    assert_eq!(value, Value::Bool(true));
                }
            }
            _ => unreachable!(),
        }

        Ok(Value::Unit)
    }

    fn report_lalrpop_error(&mut self, err: LalrpopError<'_>, input: &str) -> io::Result<()> {
        report_lalrpop_error(&mut self.stderr, err, input, &self.config)
    }

    fn report_runtime_error(&mut self, err: ErrorReport, input: &str) -> io::Result<()> {
        write_report(&mut self.stderr, err, input)
    }
}
