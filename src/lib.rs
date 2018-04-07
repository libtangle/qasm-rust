//! # QASM
//!
//! This library is a parser for the IBM OpenQASM 2.0 language.
extern crate regex;

pub mod token;
pub mod lexer;
pub mod error;
pub mod parse;

use regex::Regex;

pub fn process(input: &str) -> String {
    let cleaned = Regex::new(r"//.*").unwrap().replace_all(input, ""); // Removed All Comments
    let include_regex = Regex::new(r#"include\s*".*";"#).unwrap(); // Regex for include statments
    let processed = include_regex.replace_all(&cleaned, ""); // Remove Includes

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
