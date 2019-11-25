/// AST Nodes. These can pattern matched to evaluate the ast.
///
/// The nodes are representative of what operation should be done,
/// please look at their documentation.
#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    /// Represents the initialization of a Quantum Register.
    /// The String is the identifier, and the integer is the number of qubits.
    QReg(String, i32),
    /// Represents the initialization of a Classical Register.
    /// The String is the identifier, and the integer is the number of bits.
    CReg(String, i32),
    /// Represents a barrier to a qubit / register
    Barrier(Argument),
    /// Represents reseting a qubit / register
    Reset(Argument),
    /// Representing measuremnt of a qubit/register to a bit/register
    Measure(Argument, Argument),
    /// Represents application of a gate
    /// String is the name of the gate.
    /// The first arguments is the qubits that the gates are being applied to
    /// The second is the parameters (mathematical expressions).
    /// Note the mathematic expressions are strings, and must be evaluated
    ApplyGate(String, Vec<Argument>, Vec<String>),
    /// Represents an opaque gate
    /// String is the name of the gate.
    /// The first arguments is the qubits that the gates are being applied to
    /// The second is the parameters (mathematical expressions)
    Opaque(String, Vec<Argument>, Vec<String>),
    /// Represents the creation of a gate
    /// String is the name of the gate
    /// The first is the qubits it acts on,
    /// The seconds is the ids of the params.
    /// finally, a list of nodes, which the gate applies
    Gate(String, Vec<String>, Vec<String>, Vec<AstNode>),
    /// Represents a conditional
    /// String is classical register
    /// i32 is the value to to check if equal.
    /// If equal, AstNode is applied.
    If(String, i32, Box<AstNode>),
}

/// Representation of arguments to the ASTNodes.
/// These are never top level, thus they have been
/// left to a seperate enum.
#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    /// Represents a single qubit / bit argument.
    /// The string is the name of the register, and the integer is the index
    Qubit(String, i32),
    /// Represents a register argument.
    /// The string is the name of the register.
    Register(String),
}
