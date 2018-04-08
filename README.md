# QASM-rust

[![Build Status](https://travis-ci.org/QCGPU/qasm-rust.svg?branch=master)](https://travis-ci.org/QCGPU/qasm-rust)

> An [OPENQASM 2.0](https://arxiv.org/pdf/1707.03429.pdf) parser written in Rust 🌵

## Features

* Passes the official OpenQASM [conformance test](https://github.com/QISKit/openqasm/blob/master/contributing.md#tests) suite
* As described in [the OpenQASM specification](https://arxiv.org/pdf/1707.03429.pdf)
* Get tokens for a given source file
* Resolve include statements
* Remove comments
* Build Abstract Syntax Tree of a list of tokens

## Usage

The usage of `qasm` is fully given in the [docs](https://docs.rs/qasm/). A brief example is given here:

Here is an example that reads a file `test.qasm`, processes it and then prints the AST.

### test.qasm

```qasm
OPENQASM 2.0;

// Clifford gate: Hadamard
gate h a { u2(0,pi) a; }

qreg q[2];
creg c[1];

h q[0];
CX q[0], q[1];

measure q[1] -> c[1];
```

### main.rs

```rust
extern crate qasm;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use qasm::{process, lex, parse};

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut source = String::new();

    let mut f = File::open("test.qasm").expect("cannot find source file 'test.qasm'");
    f.read_to_string(&mut source).expect("couldn't read file 'test.qasm'");

    let processed_source = process(&source, &cwd);
    let mut tokens = lex(&processed_source);
    let ast = parse(&mut tokens);

    println!("{:?}", ast);
}
```

### Output

```rust
Ok([
    Gate("h", ["a"], [], [ApplyGate("u2", [Register("a")], [" 0 ", " pi "])]),
    QReg("q", 2),
    CReg("c", 1),
    ApplyGate("h", [Qubit("q", 0)], []),
    ApplyGate("CX", [Qubit("q", 0), Qubit("q", 1)], []),
    Measure(Qubit("q", 1), Qubit("c", 1))
])
```

## License

MIT
