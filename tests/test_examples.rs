extern crate glob;
extern crate qasm;

use glob::glob;
use qasm::{lex, parse, process};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn test_parse(source: &str) {
    let mut tokens = lex(source);

    match parse(&mut tokens) {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("Error: {}", e);
            println!("Source: {}", source);
            assert!(false)
        }
    }
}

// Start a custom repl
#[test]
fn works_with_examples() {
    for entry in glob("tests/source/*.qasm").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut f = File::open(&path).unwrap();
                let mut contents = String::new();
                f.read_to_string(&mut contents).expect("Couldn't Read File");
                contents = process(&contents, Path::new("tests/source"));

                test_parse(&contents)
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
