use std::{
    cmp::{max, min},
    io::{self, Write},
};

use ariadne::{Color, ColorGenerator, Config, Label, Report, ReportKind};
use lalrpop_util::ParseError;
use mini_c_common::{display_comma_separate_list, SushiConfig};
use mini_c_lexer::Token;

use crate::write_report;

pub type LalrpopError<'a> = ParseError<usize, Token<'a>, ()>;

pub fn report_lalrpop_error(
    writer: impl Write,
    error: LalrpopError<'_>,
    input: &str,
    &SushiConfig { is_color_enabled }: &SushiConfig,
) -> io::Result<()> {
    let report = build_report_from_lalrpop_error(error, input, is_color_enabled);
    write_report(writer, report, input)
}

fn build_report_from_lalrpop_error(
    mut error: LalrpopError<'_>,
    input: &str,
    is_color_enabled: bool,
) -> Report {
    let mut colors = ColorGenerator::new();

    reformat_lalrpop_expected_list(&mut error);

    let unset_color = |color| {
        if is_color_enabled {
            color
        } else {
            Color::Unset
        }
    };

    let [color_a, _color_b, _color_out] =
        [colors.next(), colors.next(), Color::Fixed(210)].map(unset_color);

    let config = Config::default().with_color(is_color_enabled);

    let report = Report::build(ReportKind::Error, (), 0)
        .with_config(config)
        .with_message("Failed to parse");

    let report = match error {
        ParseError::UnrecognizedEOF { location, expected } => {
            let is_semicolon_expected = expected.iter().any(|x| x == "`;`");

            let note = if is_semicolon_expected {
                "Add a `;` at the end of the line.".into()
            } else {
                let expected_tokens = display_comma_separate_list(&expected);
                format!("Found EOF, expected one of: {expected_tokens}.")
            };

            report.with_note(note).with_label(
                Label::new(location..location.saturating_add(1))
                    .with_message("Unexpected EOF")
                    .with_color(color_a),
            )
        }
        ParseError::UnrecognizedToken {
            token: (l, _, r),
            expected,
        } => {
            let span = l..r;
            let is_semicolon_expected = expected.iter().any(|x| x == "`;`");

            let note = if is_semicolon_expected {
                "Add a `;` at the end of the line.".into()
            } else {
                let expected_tokens = display_comma_separate_list(&expected);
                format!(
                    "Found `{}`, expected one of: {expected_tokens}.",
                    &input[span.clone()]
                )
            };

            report.with_note(note).with_label(
                Label::new(span)
                    .with_message("Unexpected token")
                    .with_color(color_a),
            )
        }
        ParseError::InvalidToken { .. } | ParseError::ExtraToken { .. } => unreachable!(),
        ParseError::User { .. } => unreachable!("We don't have custom parser errors yet"),
    };

    // x.print(Source::from(include_str!("sample.tao"))).unwrap();

    report.finish()
}

fn reformat_lalrpop_expected_list(error: &mut LalrpopError) {
    fn replace_quotes_by_backticks(vec: &mut [String]) {
        for token in vec.iter_mut() {
            let len = token.len();
            assert!(len > 0);
            unsafe {
                token.as_bytes_mut()[0] = b'`';
                token.as_bytes_mut()[len - 1] = b'`';
            }
        }
    }

    fn simplify_bool(vec: &mut Vec<String>) {
        let true_index = vec.iter().position(|token| token == "`true`");
        let false_index = vec.iter().position(|token| token == "`false`");

        let (Some(true_index), Some(false_index)) = (true_index, false_index) else {
            return;
        };

        let min = min(true_index, false_index);
        let max = max(true_index, false_index);

        vec.remove(max);
        vec.remove(min);
        vec.insert(min, "`{bool}`".into());
    }

    fn simplify_literal(vec: &mut Vec<String>) {
        let int_index = vec.iter().position(|token| token == "`{int}`");
        let bool_index = vec.iter().position(|token| token == "`{bool}`");

        let (Some(int_index), Some(bool_index)) = (int_index, bool_index) else {
            return;
        };

        let min = min(int_index, bool_index);
        let max = max(int_index, bool_index);

        vec.remove(max);
        vec.remove(min);
        vec.insert(min, "`{literal}`".into());
    }

    if let LalrpopError::UnrecognizedEOF { expected, .. }
    | LalrpopError::UnrecognizedToken { expected, .. } = error
    {
        replace_quotes_by_backticks(expected);
        expected.reverse();
        simplify_bool(expected);
        simplify_literal(expected);
    }
}
