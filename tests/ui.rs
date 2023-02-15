#![feature(custom_inner_attributes)]

use unindent::unindent;

macro_rules! test {
    ($input:expr) => {
        insta::assert_display_snapshot!($input)
    };
}

#[test]
fn file_1() {
    test!(unindent(
        r#"
    "#
    ));
}

// #[test]
// fn file_2() {
//     test!(unindent(
//         r#"
//     "#
//     ));
// }
