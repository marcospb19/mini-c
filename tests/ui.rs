//! This is a test module that creates snapshots, and might do
//! UI testing in the future.
//!
//! This shouldn't be called ui.rs hmmmmmmmm.
//!
//! Snapshot todo list:
//! - [X] Source code.
//! - [X] AST.
//! - [ ] LLVM IR.
//! - [ ] Execution stdout output (UI).
//! - [ ] (maybe) Execution final state (variables, functions).

use fs_err as fs;
use mini_c::test_utils;

macro_rules! snapshot_example_file {
    ($example_path:literal) => {
        let file_contents = &fs::read_to_string($example_path)
            .unwrap_or_else(|err| panic!("Failed to read file {}: {err}.", $example_path));
        // Source code
        insta::assert_display_snapshot!(file_contents);
        // AST
        insta::assert_ron_snapshot!(test_utils::generate_ast(file_contents));
    };
}

#[test]
fn example_break() {
    snapshot_example_file!("examples/break.c");
}
#[test]
fn example_coins() {
    snapshot_example_file!("examples/coins.c");
}
#[test]
fn example_conds() {
    snapshot_example_file!("examples/conds.c");
}
#[test]
fn example_easy() {
    snapshot_example_file!("examples/easy.c");
}
#[test]
fn example_fib() {
    snapshot_example_file!("examples/fib.c");
}
#[test]
fn example_file_1() {
    snapshot_example_file!("examples/file_1.c");
}
#[test]
fn example_file_2() {
    snapshot_example_file!("examples/file_2.c");
}
#[test]
fn example_if_pass() {
    snapshot_example_file!("examples/if-pass.c");
}
#[test]
fn example_operators() {
    snapshot_example_file!("examples/operators.c");
}
#[test]
fn example_queen() {
    snapshot_example_file!("examples/queen.c");
}
#[test]
fn example_var() {
    snapshot_example_file!("examples/var.c");
}
#[test]
fn example_var_loop() {
    snapshot_example_file!("examples/var_loop.c");
}
#[test]
fn example_while() {
    snapshot_example_file!("examples/while.c");
}
