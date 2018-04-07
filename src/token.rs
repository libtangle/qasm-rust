#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// This token represents an illegal token.
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
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,
    Pi,

    // Built In Gates

    // Operators
    QReg,
    CReg,
    Barrier,
    Gate,
    Measure,
    Reset,
    Include,
    Opaque,
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
