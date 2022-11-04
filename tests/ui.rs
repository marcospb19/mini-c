#![feature(custom_inner_attributes)]
#![rustfmt::skip]

use sushi::test_sushi;
use unindent::unindent;

#[test]
fn empty_statements() {
    let input = unindent("
        ;
        ;;
    ");
    insta::assert_debug_snapshot!(test_sushi(input));
}

#[test]
fn empty_program() {
    let input = "";
    insta::assert_debug_snapshot!(test_sushi(input));
}

#[test]
fn arithmetic_expression() {
    let input = "1 + 1 * 2 + 3 / (5 - 3);";
    insta::assert_debug_snapshot!(test_sushi(input));
}
