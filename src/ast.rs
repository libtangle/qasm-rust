#[derive(Debug, PartialEq)]
pub enum Expr {
    QReg(String, i32),
    CReg(String, i32),
    /// Reset a qubit or a register
    Reset(Argument),
    Measure(Argument, Argument),
    Barrier(Vec<Argument>),
    Application(String, Vec<Argument>, Vec<Box<MathExp>>),
    Opaque(String, Vec<Argument>, Vec<Box<MathExp>>),
    Conditional(String, i32, Box<Expr>),
    Gate(String, Vec<String>, Vec<String>, Vec<Box<Expr>>)
}

/// Representations of arguemnts in QASM
#[derive(Debug, PartialEq)]
pub enum Argument {
    /// A whole register is the argument
    /// the parameter is the name of the register
    Register(String),
    /// A single qubit is the argument,
    /// the first parameter is the name of the register
    /// and the second is the index of the qubit im the register
    Qubit(String, i32),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    /// The version of OPENQASM to use, this only supports version 2.0, so an error will be
    /// thrown if the version is not 2.0
    pub version: f32,
    pub body: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum MathExp {
    Number(f32),
    Variable(String),
    Unary(UnaryOp, Box<MathExp>),
    Binary(Box<MathExp>, BinaryOp, Box<MathExp>),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    // Binary Operators
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    // Unary Operators
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,
    Neg, // Negates a number
}
