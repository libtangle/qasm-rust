extern crate qasm;

use qasm::{lex, parse, process};
use parse::parse;
use std::path::Path;

// Start a custom repl
fn main() {
    let input = include_str!("test.qasm");
    let cwd = Path::new(file!()).parent().unwrap();

    let processed = process(input, cwd);
    let mut tokens = lex(&processed);

    match parse(&mut tokens) {
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
