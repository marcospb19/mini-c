#![feature(custom_inner_attributes)]
#![rustfmt::skip]

use sushi::test_sushi;
use unindent::unindent;

macro_rules! test {
    ($input:expr) => {
        insta::assert_display_snapshot!(test_sushi($input).unwrap())
    };
}

#[test]
fn empty_statements() {
    test!(unindent("
        ;
        ;;
    "));
}

#[test]
fn empty_program() {
    test!("");
}

#[test]
fn arithmetic_expression() {
    test!("1 + 1 * 2 + 3 / (5 - 3);");
}

#[test]
fn missing_trailing_semi_colon() {
    test!("1 + 1 * 2 + 3 / (5 - 3)");
}

#[test]
fn unfinished_arithmetic_expression() {
    test!("1 -;");
}
