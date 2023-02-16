use ariadne::{Color, ColorGenerator, Label, Report, ReportKind};
use mini_c_ast::{Span, Spanned, Value};

use crate::ErrorReport;

pub fn binary_operator_error(
    left: Spanned<Value>,
    right: Spanned<Value>,
    operator_span: Span,
) -> ErrorReport {
    let mut colors = ColorGenerator::new();

    let [color_1, color_2, color_3] = [colors.next(), colors.next(), Color::Fixed(210)];

    let lhs_type_info = format!("Type is {}", left.inner.value_type_str());
    let rhs_type_info = format!("Type is {}", right.inner.value_type_str());

    let report = Report::build(ReportKind::Error, (), 0)
        .with_message("Failed to do binary operation")
        .with_note("operation is not supported for the provided types")
        .with_label(
            Label::new(left.span.into())
                .with_message(lhs_type_info)
                .with_color(color_1),
        )
        .with_label(
            Label::new(operator_span.into())
                .with_message("operator is not supported")
                .with_color(color_2),
        )
        .with_label(
            Label::new(right.span.into())
                .with_message(rhs_type_info)
                .with_color(color_3),
        );

    report.finish()
}
