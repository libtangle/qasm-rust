extern crate qasm;
use qasm::{eval, parse};

pub fn main() {
    let ast = parse(include_str!("test.qasm"));

    println!("{:?}", ast);
    eval(ast);
}
