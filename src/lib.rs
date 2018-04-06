//! # QASM
//!
//! This library is a parser for the IBM OpenQASM 2.0 language.
//! It is written using the LALRPOP parser generator.
//!
//! ## Usage
//!
//! Call the parser with some source code
//!
//! ```rust
//! # extern crate qasm;
//! qasm::parse(r#"
//! OPENQASM 2.0;
//! qreg register[3];
//!"#);
//! ```
//!
//! This will return an abstract syntax tree.

extern crate regex;
pub mod qasm; // synthesized by LALRPOP
pub mod ast;

use regex::Regex;

/// The main parser. This will return a `program` struct
///
/// ## Usage
/// ```rust
/// # extern crate qasm;
/// qasm::parse(r#"
/// OPENQASM 2.0;
/// qreg register[3];
/// "#);
/// ```
pub fn parse(source: &str) -> ast::Program {
    // Lalrpop cannot parse comments, so the easiest
    // thing to do without implementing a custom lexer
    // is just to preprocess the source code to
    // 1. Remove any comments
    // 2. Handle any 'include' statements
    let cleaned = Regex::new(r"//.*").unwrap().replace_all(source, ""); // Removed All Comments
    let include_regex = Regex::new(r#"include\s*".*";"#).unwrap(); // Regex for include statments
    let processed = include_regex.replace_all(&cleaned, ""); // Remove Includes

    println!("{}", processed);
    qasm::ProgramParser::new().parse(&processed).unwrap()
}

pub fn eval(tree: ast::Program) {
    for expr in tree.body {
        match expr {
            ast::Expr::CReg(name, val) => {
                println!("Creating a classical register: {} with {} bits", name, val)
            }
            ast::Expr::QReg(name, val) => {
                println!("Creating a quantum register: {} with {} qubits", name, val)
            }
            ast::Expr::Reset(arg) => {
                println!("Resetting {:?}", arg)
            }
            ast::Expr::Measure(arg_1, arg_2) => {
                println!("Measuring {:?} to {:?}", arg_1, arg_2)
            }
            ast::Expr::Barrier(args) => {
                println!("Barrier on {:?}", args)
            }
            ast::Expr::Application(name, args, params) => {
                println!("Applying {} with parameters: {:?} to {:?}", name, params, args)
            }
            ast::Expr::Opaque(name, args, params) => {
                println!("Declared an opaque gate {} with parameters: {:?} to {:?}", name, params, args)
            }
            ast::Expr::Conditional(name, n, expr) => {
                println!("If {:?} has the value {}, then {:?}", name, n, expr)
            }
            ast::Expr::Gate(name, params, qargs, exprs) => {
                println!("Declared gate {} with parameters {:?} and arguments {:?}. \nThis gate applied {:?}", name, params, qargs, exprs)
            }
        }
    }
}
