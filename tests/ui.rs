#![feature(custom_inner_attributes)]
#![rustfmt::skip]

use sushi::test_utils::test_sushi;
use unindent::unindent;

macro_rules! test {
    ($input:expr) => {
        insta::assert_display_snapshot!(test_sushi($input))
    };
}

#[test]
fn empty_statements() {
    test!(unindent("
        ;  ; ;;
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

#[test]
fn print_calls() {
    test!(unindent("
        print(1, 2, 3);
        print(5 + 5 * 10);
        print(true);
        print(false);
        print(());
        print((), ());
    "));
}

#[test]
fn integer_comparisons() {
    test!(unindent("
        assert(1 + 2 == 3);
        assert(1 + 2 != 4);
        assert(1 < 2);
        assert(not (1 < 1));
        assert(1 <= 1);
        assert(1 == 1);
        assert(not (1 <= 0));
    "));
}

#[test]
fn store_variable_value() {
    test!(unindent("
        let x = 10;
        let y = x + 10;
        let z = y;
        assert(z == 20);
    "));
}

#[test]
#[ignore = "refac em sushi-errorr deveria acontecer antes, pq do jeito que to adicionando esses erros, tá estranho"]
fn undefined_variable() {
 test!("x;");
}
