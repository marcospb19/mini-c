mod binary_operation;
mod lalrpop;
mod unary_operation;

use std::io::{self, Write};

use ariadne::Source;

pub use self::{
    binary_operation::binary_operator_error,
    lalrpop::{report_lalrpop_error, LalrpopError},
    unary_operation::unary_operator_error,
};

pub type ErrorReport = ariadne::Report;

pub type RuntimeResult<T = ()> = std::result::Result<T, ErrorReport>;

pub fn write_report(writer: impl Write, report: ErrorReport, input: &str) -> io::Result<()> {
    report.write(Source::from(input), writer)
}
