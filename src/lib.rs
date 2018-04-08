//! # QASM
//!
//! This library is a parser for the IBM OpenQASM 2.0 language.
//!
//! It is seperated into 3 parts:
//! 1. Processing - Removing comments and resolving include statements.
//! 2. Lexing - Splitting up the processed source file into a list of tokens (`Vec<Token>`)
//! 3. Parsing - Turning the list of tokens into a list of AST nodes.
//!
//! There is methods provided for each.
//!
//! ## Processing
//!
//! Processing is done with the [processing](fn.process.html) function.
//! It requires 2 arguments, the input string, and the `Path` of the directory it
//! is in. This path is used to locate the include files.
//! It is used like so:
//!
//! ```no_run
//! extern crate qasm;
//! use std::env;
//!
//! let source = r#"
//! OPENQASM 2.0;
//! // Here is a comment
//! include "sample.inc";
//!
//! qreg a[3];
//! // And so on
//! "#;
//!
//! let cwd = env::current_dir().unwrap();
//! qasm::process(source, &cwd);
//! /* Will Return:
//!  *
//!  * ```
//!  * OPENQASM 2.0;
//!  *
//!  * (CONTENTS OF SAMPLE.INC)
//!  *
//!  * qreg a[3];
//!  * ```
//!  */
//!
//! ```
//!
//! ## Lexing
//!
//! Lexing is done with the [lex](fn.lex.html) function.
//! It takes a source string (which must not have any comments or include statements)
//! and returns a Vector of [Token](enum.Token.html)s.
//!
//! It is used like so:
//!
//! ```rust
//! extern crate qasm;
//!
//! let source = r#"
//! OPENQASM 2.0;
//! qreg a[3];
//! CX a[0], a[1];
//! "#;
//!
//! let tokens = qasm::lex(source);
//! println!("{:?}", tokens);
//! // [OpenQASM, Real(2.0), Semicolon,
//! //  QReg, Id("a"), LSParen, NNInteger(3), RSParen, Semicolon,
//! //  Id("CX"), Id("a"), LSParen, NNInteger(0), RSParen, Comma, Id("a"), LSParen, NNInteger(1), RSParen, Semicolon]
//! ```
//!
//! for a full list of tokens that can be returned, please see the [Token](enum.Token.html) enum.
//!
//! ## Parsing
//! Parsing is done with the [parse](fn.parse.html) function. It accepts a vector of [Token](enum.Tokem.html)s
//! and returns a vector of [AstNode](enum.AstNode.html)s or an [Error](enum.Error.html) as a result
//!
//!
//! It is used like so:
//! ```rust
//! extern crate qasm;
//! use qasm::Token;
//!
//! let mut tokens = vec![
//!     Token::OpenQASM,
//!     Token::Real(2.0),
//!     Token::Semicolon,
//!     Token::QReg,
//!     Token::Id("a".to_string()),
//!     Token::LSParen,
//!     Token::NNInteger(3),
//!     Token::RSParen,
//!     Token::Semicolon,
//!     Token::Id("CX".to_string()),
//!     Token::Id("a".to_string()),
//!     Token::LSParen,
//!     Token::NNInteger(0),
//!     Token::RSParen,
//!     Token::Comma,
//!     Token::Id("a".to_string()),
//!     Token::LSParen,
//!     Token::NNInteger(1),
//!     Token::RSParen,
//!     Token::Semicolon,
//! ];
//! let ast = qasm::parse(&mut tokens);
//!
//! // Ok([QReg("a", 3), ApplyGate("CX", [Qubit("a", 0), Qubit("a", 1)], [])])
//! ```
//!
//! ## Combining Functions
//! The functions can be combined to process, lex and parse a source string.
//! Here is an example that reads a file 'test.qasm', processes it and then prints the AST.
//!
//! ### test.qasm
//!
//! ```qasm
//! OPENQASM 2.0;
//!
//! // Clifford gate: Hadamard
//! gate h a { u2(0,pi) a; }
//!
//! qreg q[2];
//! creg c[1];
//!
//! h q[0];
//! CX q[0], q[1];
//!
//! measure q[1] -> c[1];
//! ```
//!
//! ### main.rs
//!
//! ```no_run
//! extern crate qasm;
//!
//! use std::env;
//! use std::fs::File;
//! use std::io::prelude::*;
//! use qasm::{process, lex, parse};
//!
//! fn main() {
//!     let cwd = env::current_dir().unwrap();
//!     let mut source = String::new();
//!
//!     let mut f = File::open("test.qasm").expect("cannot find source file 'test.qasm'");
//!     f.read_to_string(&mut source).expect("couldn't read file 'test.qasm'");
//!
//!     let processed_source = process(&source, &cwd);
//!     let mut tokens = lex(&processed_source);
//!     let ast = parse(&mut tokens);
//!
//!     println!("{:?}", ast);
//! }
//! ```
//!
//! ### Output
//!
//! ```rust,ignore
//! Ok([
//!     Gate("h", ["a"], [], [ApplyGate("u2", [Register("a")], [" 0 ", " pi "])]),
//!     QReg("q", 2),
//!     CReg("c", 1),
//!     ApplyGate("h", [Qubit("q", 0)], []),
//!     ApplyGate("CX", [Qubit("q", 0), Qubit("q", 1)], []),
//!     Measure(Qubit("q", 1), Qubit("c", 1))
//! ])
//! ```
extern crate regex;

mod token;
mod lexer;
mod error;
mod parser;
mod ast;

use std::fs::File;
use std::io::prelude::*;
use regex::{Captures, Regex};
use std::path::Path;

pub use error::Error;
pub use ast::Argument;
pub use ast::AstNode;
pub use token::Token;

type Result<T> = std::result::Result<T, Error>;

/// Remove comments from an input string and resolves include statements.
///
/// This function has 2 arguments, the input string, and the path that the file is in.
/// The path is used to resolve the include statements.
///
/// This function returns a String that has no comments and no include statements.
/// The include statements will have been replaced by the text of the include file.
///
/// This function will panic when the included file doesn't exist or when it couldn't be read.
///
/// ## Example
/// ```no_run
/// extern crate qasm;
/// use std::env;
///
/// let source = r#"
/// OPENQASM 2.0;
/// // Here is a comment
/// include "sample.inc";
///
/// qreg a[3];
/// // And so on
/// "#;
///
/// let cwd = env::current_dir().unwrap();
/// qasm::process(source, &cwd);
/// /* Will Return:
///  *
///  * ```
///  * OPENQASM 2.0;
///  *
///  * (CONTENTS OF SAMPLE.INC)
///  *
///  * qreg a[3];
///  * ```
///  */
/// ```
pub fn process(input: &str, cwd: &Path) -> String {
    let comment_regex = Regex::new(r"//.*").unwrap();
    let cleaned = comment_regex.replace_all(input, ""); // Removed All Comments

    let include_regex = Regex::new(r#"include\s*"(?P<s>.*)";"#).unwrap(); // Regex for include statments

    let replace_with_file = |caps: &Captures| {
        let path = cwd.join(&caps["s"]);

        let mut f = File::open(path).expect("Couldn't Open An Include File");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Couldn't Read Include Statement");
        comment_regex.replace_all(&contents, "").into()
    };

    let processed = include_regex.replace_all(&cleaned, replace_with_file); // Remove Includes

    processed.into()
}

/// Take a source string with no includes or comments and returns the tokens
///
/// The source string can be processed with the [process](fn.process.html) function.
/// The tokens are all varients of [Token](enum.Token.html). An illegal token will be returned
/// for any unrechognised tokens.
///
/// ## Examples
///
/// ```rust
/// extern crate qasm;
///
/// let source = r#"
/// OPENQASM 2.0;
/// qreg a[3];
/// CX a[0], a[1];
/// "#;
///
/// let tokens = qasm::lex(source);
/// println!("{:?}", tokens);
/// // [OpenQASM, Real(2.0), Semicolon,
/// //  QReg, Id("a"), LSParen, NNInteger(3), RSParen, Semicolon,
/// //  Id("CX"), Id("a"), LSParen, NNInteger(0), RSParen, Comma, Id("a"), LSParen, NNInteger(1), RSParen, Semicolon]
/// ```
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

/// Changes a vector of tokens into an AST.
///
/// Parsing is done with the [parse](fn.parse.html) function. It accepts a vector of [Token](enum.Tokem.html)s
/// and returns a vector of [AstNode](enum.AstNode.html)s or an [Error](enum.Error.html) as a result
///
/// ## Example
///
/// ```rust
/// extern crate qasm;
///
/// let mut tokens = vec![
///     qasm::Token::OpenQASM,
///     qasm::Token::Real(2.0),
///     qasm::Token::Semicolon,
///     qasm::Token::QReg,
///     qasm::Token::Id("a".to_string()),
///     qasm::Token::LSParen,
///     qasm::Token::NNInteger(3),
///     qasm::Token::RSParen,
///     qasm::Token::Semicolon,
///     qasm::Token::Id("CX".to_string()),
///     qasm::Token::Id("a".to_string()),
///     qasm::Token::LSParen,
///     qasm::Token::NNInteger(0),
///     qasm::Token::RSParen,
///     qasm::Token::Comma,
///     qasm::Token::Id("a".to_string()),
///     qasm::Token::LSParen,
///     qasm::Token::NNInteger(1),
///     qasm::Token::RSParen,
///     qasm::Token::Semicolon,
/// ];
/// let ast = qasm::parse(&mut tokens);
///
/// // Ok([QReg("a", 3), ApplyGate("CX", [Qubit("a", 0), Qubit("a", 1)], [])])
/// ```
pub fn parse(tokens: &mut Vec<token::Token>) -> Result<Vec<AstNode>> {
    let mut nodes = vec![];

    // Check that the version is first, and that it is version 2.0
    if tokens.remove(0) != token::Token::OpenQASM {
        return Err(Error::MissingVersion);
    }
    if parser::version(tokens)? != 2.0 {
        return Err(Error::UnsupportedVersion);
    }

    while !tokens.is_empty() {
        let node = parser::parse_node(tokens)?;
        nodes.push(node);
    }

    Ok(nodes)
}
