use std::io::{stdin, Read};

use sushi::run_sushi;

fn main() {
    let input = {
        let mut string = String::new();
        let _bytes = stdin().read_to_string(&mut string).unwrap();
        string
    };

    run_sushi(&input).unwrap();
}
