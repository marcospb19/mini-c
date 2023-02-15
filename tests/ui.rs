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
        #include "minicio.h"

        int max(int a, int b) {
            if (a > b) {
                return a;
            } else {
                return b;
            }
        }

        int main() {
            int a = getint();
            int b = getint();
            int maximum = max(a, b);

            putint(maximum);
            putnewline();

            return 0;
        }
    "#
    ));
}

#[test]
fn file_2() {
    test!(unindent(
        r#"
        #include "minicio.h"

        int mod(int a, int b) {
            return a - (a / b) * b;
        }

        int gcd(int a, int b) {
            if (b == 0) {
                return a;
            } else {
                return gcd(b, mod(a, b));
            }
        }

        int main() {
            int a = getint();
            int b = getint();
            int divisor = gcd(a, b);

            putint(divisor);
            putnewline();

            return 0;
        }
    "#
    ));
}
