extern crate qasm;

use qasm::{lex, parse, process};
use parse::parse;
use std::io::{self, BufRead, Write};

// Start a custom repl
fn main() {
    let stdin = io::stdin();

    loop {
        // Stdout needs to be flushed, due to missing newline
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Error reading from stdin");
        line = process(&line);
        let mut tokens = lex(&line);

        println!("{:?}", parse(&mut tokens));
    }
}
