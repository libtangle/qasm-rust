//! # QASM
//!
//! This library is a parser for the IBM OpenQASM 2.0 language.
extern crate regex;

pub mod token;
pub mod lexer;

use regex::Regex;

pub fn process(input: &str) -> String {
    let cleaned = Regex::new(r"//.*").unwrap().replace_all(input, ""); // Removed All Comments
    let include_regex = Regex::new(r#"include\s*".*";"#).unwrap(); // Regex for include statments
    let processed = include_regex.replace_all(&cleaned, ""); // Remove Includes

    processed.into()
}
