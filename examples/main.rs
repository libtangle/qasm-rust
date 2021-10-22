extern crate qasm;

use qasm::{lex, parse, process};
use std::path::Path;

// Start a custom repl
fn main() {
    let input = include_str!("qft.qasm");
    let cwd = Path::new(file!()).parent().unwrap();

    let processed = process(input, cwd);
    let tokens = lex(&processed);

    match parse(&tokens) {
        Ok(ast) => {
            println!("AST: {:?}", ast);
            println!("\x1b[32mAll Okay!\x1b[0m");
        }
        Err(e) => {
            println!("\x1b[31mGot an error: {}\x1b[0m", e);
            println!("{:?}", tokens)
        }
    }
}
