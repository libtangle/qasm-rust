//! # QASM
//!
//! This library is a parser for the IBM OpenQASM 2.0 language.
extern crate regex;

pub mod token;
pub mod lexer;
pub mod error;
pub mod parse;

use std::fs::File;
use std::io::prelude::*;
use regex::{Captures, Regex};
use std::path::Path;

pub fn process(input: &str, cwd: &Path) -> String {
    let comment_regex = Regex::new(r"//.*").unwrap();
    let cleaned = comment_regex.replace_all(input, ""); // Removed All Comments

    let include_regex = Regex::new(r#"include\s*"(?P<s>.*)";"#).unwrap(); // Regex for include statments

    let replace_with_file = |caps: &Captures| {
        let path = cwd.join(&caps["s"]);

        let mut f = File::open(path).expect("Couldn't Open An Include File");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Couldn't Read Include Statement");
        comment_regex.replace_all(&contents, "").into()
    };

    let processed = include_regex.replace_all(&cleaned, replace_with_file); // Remove Includes

    processed.into()
}

pub fn lex(input: &str) -> Vec<token::Token> {
    let mut lexer = lexer::Lexer::new(input);
    let mut tokens = vec![];

    loop {
        let tok = lexer.next_token();
        if tok == token::Token::EndOfFile {
            break;
        }
        tokens.push(tok);
    }

    tokens
}
