use std::{fs, path::PathBuf};

use mini_c_lexer::Lexer;
use mini_c_llvm_codegen::Compiler;
use mini_c_parser::ProgramParser;

fn main() {
    let files = get_file_args();

    for path in files {
        let input = fs::read_to_string(&path).expect("Failed to read file");

        let lexer = Lexer::new(&input);
        let ast = ProgramParser::new()
            .parse(&input, lexer)
            .expect("Failed to parse program");

        let mut compiler = Compiler::new();
        compiler.compile(&ast).expect("Failed to compile program");
        compiler.run();
    }
}

fn get_file_args() -> Vec<PathBuf> {
    // Run on all examples
    let argv = std::env::args()
        .skip(1)
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();

    match argv.as_slice() {
        [] => {
            glob::glob("examples/*")
                .unwrap()
                .map(Result::unwrap)
                .collect()
        }
        _ => argv,
    }
}

// A4 Error statements
// The error statements that your program should print out is below. After the error statement, the compiler should show which line and column the error occured in parenthesis in the same line:
// Function has return type "y", but the returned expression has type "x"! (line:col)
// e.g. Function has return type "bool", but the returned expression has type "int"! (13:8)

// The provided public tester also shows the correct format of error statements, but the order of the statements do not matter.
// You have to replace the placeholders (name, x, y) with the correct names in the code as well.

// Error statements:

// Redefinition of variable/parameter "name" in the same scope!
// Definition of function "name()" with different return type!
// Definition of function "name()" with different number of parameters!
// Definition of function "name()" with different parameter type at position x!
// Redefinition of function "name()"!

// The function "name()" need to return a value at its end!
// Function has void return type, but the return statement has a returned expression!
// Function has non-void return type, but the return statement has no returned expression!
// Function has return type "y", but the returned expression has type "x"!

// Break statement must appear inside a for/while statement!

// Conditional expression in if statement has non-bool type!
// Conditional expression in for statement has non-bool type!
// Conditional expression in while statement has non-bool type!
// Negate "-" opcode must have int operand!
// Not "!" opcode must have bool operand!
// "&&"/"||" opcode must have bool operand!
// "=="/"!=" opcode must have same primitive type operand!
// ">" opcode must have int type operand!
// "<" opcode must have int type operand!
// ">=" opcode must have int type operand!
// "<=" opcode must have int type operand!
// Variable name is not declared before use!
// Array index expressions must have int operand!
// Indexing an non-array variable!
// Variable and the assignment expression do not have the same type!
// Integer literal must be inside the range of int!
// "+" opcode must have int type operand!
// "-" opcode must have int type operand!
// "*" opcode must have int type operand!
// "/" opcode must have int type operand!

// Function name() is not declared before use!
// Function name() is declared with x parameters but called with y arguments!
// Function name() does not match the type of the call argument at position x!
