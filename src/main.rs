use std::io::{read_to_string, stdin};

use sushi::run_sushi;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    run_sushi(&input).unwrap();
}
