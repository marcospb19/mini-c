mod lalrpop;

use std::io::{self, Write};

#[derive(Debug)]
pub enum Error {}

pub type RuntimeResult<T = ()> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {}

use ariadne::{Report, Source};

pub use self::lalrpop::*;

pub fn write_report(writer: impl Write, report: Report, input: &str) -> io::Result<()> {
    report.write(Source::from(input), writer)
}
