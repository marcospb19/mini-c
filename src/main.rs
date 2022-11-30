use sushi::run_sushi_repl;

fn main() {
    run_sushi_repl().unwrap_or_else(|err| {
        eprintln!("Error: {err}.");
        std::process::exit(1);
    });
}
