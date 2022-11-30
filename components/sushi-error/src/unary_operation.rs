use ariadne::{ColorGenerator, Label, Report, ReportKind};
use sushi_ast::{Span, Spanned, Value};

use crate::ErrorReport;

pub fn unary_operator_error(value: Spanned<Value>, operator_span: Span) -> ErrorReport {
    let mut colors = ColorGenerator::new();

    let [color_1, color_2] = [colors.next(), colors.next()];

    let value_type_info = format!("Type is {}", value.inner.value_type_str());

    let report = Report::build(ReportKind::Error, (), 0)
        .with_message("Failed to do unary operation")
        .with_note("operation is not supported for the provided type")
        .with_label(
            Label::new(value.span.into())
                .with_message(value_type_info)
                .with_color(color_1),
        )
        .with_label(
            Label::new(operator_span.into())
                .with_message("operator is not supported")
                .with_color(color_2),
        );

    report.finish()
}
