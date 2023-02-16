// mod unary_operation;
// mod binary_operation;
mod lalrpop;

use std::io::{self, Write};

use ariadne::Source;

// unary_operation::unary_operator_error,
// binary_operation::binary_operator_error,
pub use self::lalrpop::{report_lalrpop_error, LalrpopError};

pub type ErrorReport = ariadne::Report;

pub type RuntimeResult<T = ()> = std::result::Result<T, ErrorReport>;

pub fn write_report(writer: impl Write, report: ErrorReport, input: &str) -> io::Result<()> {
    report.write(Source::from(input), writer)
}
