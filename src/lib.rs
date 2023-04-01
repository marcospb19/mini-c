pub mod test_utils {
    use mini_c_ast::Program;
    use mini_c_lexer::Lexer;
    use mini_c_parser::ProgramParser;

    pub fn generate_ast(input: impl AsRef<str>) -> Program {
        let input = input.as_ref();

        let lexer = Lexer::new(input);
        let ast = ProgramParser::new().parse(input, lexer);

        match ast {
            Ok(inner) => inner,
            Err(err) => {
                dbg!(err);
                panic!("failing here, TODO this is kinda wrong.");
            }
        }
    }
}
