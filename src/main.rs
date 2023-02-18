use std::fs;

use mini_c_lexer::Lexer;
use mini_c_parser::ProgramParser;

fn main() {
    let examples = glob::glob("examples/*").unwrap();

    for path in examples {
        let path = path.unwrap();
        let input = fs::read_to_string(&path).unwrap();

        let lexer = Lexer::new(&input);
        let ast = ProgramParser::new().parse(&input, lexer);

        if let Err(err) = ast {
            dbg!(err);
            println!("failing on = {}", path.display());
            return;
        }
    }

    println!("all succeeded");
}
