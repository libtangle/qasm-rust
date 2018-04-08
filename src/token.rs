/// Tokens returned from lexing. Represents a small amount of the source code.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// This token represents an illegal token. This is usually an error in the source code.
    Illegal,
    /// This token represents the end of a file.
    EndOfFile,

    // Literals
    /// Represents a Real Number
    Real(f32),
    /// Represents an integer
    NNInteger(i32),
    /// Represents an identifier
    Id(String),

    // Other Tokens
    /// The OPENQASM statement
    OpenQASM,
    /// A Semicolon
    Semicolon,
    /// A Comma
    Comma,
    /// A Left Paren `(`
    LParen,
    /// A Left Square Paren `[`
    LSParen,
    /// A Left Curly Paren `{`
    LCParen,
    /// A Right Paren `)`
    RParen,
    /// A Right Square Paren `]`
    RSParen,
    /// A Right Curly Paren `}`
    RCParen,
    /// An Arrow `->`
    Arrow,
    /// An Equals `==`
    Equals,

    // Mathematical Expressions
    /// Plus Sign `+`
    Plus,
    /// Minus Sign `-`
    Minus,
    /// Times Sign `*`
    Times,
    /// Divide Sign `/`
    Divide,
    /// Power Sign `^`
    Power,
    /// Unary Sin function
    Sin,
    /// Unary Cos function
    Cos,
    /// Unary Tan function
    Tan,
    /// Unary exp function (e^x)
    Exp,
    /// Unary natural logarithm function
    Ln,
    /// Unary square root function
    Sqrt,
    /// Pi (3.1415....)
    Pi,

    // Built In Gates

    // Operators
    /// Reserved word, `qreg`
    QReg,
    /// Reserved word, `creg`
    CReg,
    /// Reserved word, `barrier`
    Barrier,
    /// Reserved word, `gate`
    Gate,
    /// Reserved word, `measure`
    Measure,
    /// Reserved word, `reset`
    Reset,
    /// Reserved word, `include`
    Include,
    /// Reserved word, `opaque`
    Opaque,
    /// Reserved word, `if`
    If,
}

impl Default for Token {
    /// Choose the Illegal token as default
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "qreg" => Token::QReg,
        "creg" => Token::CReg,
        "barrier" => Token::Barrier,
        "gate" => Token::Gate,
        "measure" => Token::Measure,
        "reset" => Token::Reset,
        "include" => Token::Include,
        "opaque" => Token::Opaque,
        "if" => Token::If,
        "sin" => Token::Sin,
        "cos" => Token::Cos,
        "tan" => Token::Tan,
        "exp" => Token::Exp,
        "ln" => Token::Ln,
        "sqrt" => Token::Sqrt,
        "pi" => Token::Pi,
        "OPENQASM" => Token::OpenQASM,
        _ => Token::Id(ident.into()),
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(lookup_ident("opaque"), Token::Opaque);
}
