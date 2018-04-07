use token::Token;
use error::Error;
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    /// Represents the initialization of a Quantum Register.
    /// The String is the identifier, and the integer is the number of qubits.
    QReg(String, i32),
    /// Represents the initialization of a Classical Register.
    /// The String is the identifier, and the integer is the number of qubits.
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

#[derive(Debug, PartialEq)]
pub enum Argument {
    /// Represents a single qubit / bit argument.
    /// The string is the name of the register, and the integer is the index
    Qubit(String, i32),
    /// Represents a register argument.
    /// The string is the name of the register.
    Register(String),
}

fn parse_node(tokens: &mut Vec<Token>) -> Result<AstNode> {
    match tokens.remove(0) {
        Token::QReg => qreg(tokens),
        Token::CReg => creg(tokens),
        Token::Barrier => barrier(tokens),
        Token::Reset => reset(tokens),
        Token::Measure => measure(tokens),
        Token::Id(i) => application(tokens, i),
        Token::Opaque => opaque(tokens),
        Token::Gate => gate(tokens),
        Token::If => if_(tokens),

        // These tokens shouldn't come up
        Token::Illegal
        | Token::OpenQASM
        | Token::Semicolon
        | Token::Comma
        | Token::LParen
        | Token::LCParen
        | Token::LSParen
        | Token::RCParen
        | Token::RParen
        | Token::Arrow
        | Token::Real(_)
        | Token::NNInteger(_)
        | Token::Equals
        | Token::Plus
        | Token::Minus
        | Token::Divide
        | Token::Times
        | Token::Power
        | Token::Sin
        | Token::Cos
        | Token::Tan
        | Token::Sqrt
        | Token::Exp
        | Token::Ln
        | Token::Pi
        | Token::RSParen => return Err(Error::SourceError),
        _ => return Err(Error::SourceError),
    }
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Vec<AstNode>> {
    let mut nodes = vec![];

    // Check that the version is first, and that it is version 2.0
    if tokens.remove(0) != Token::OpenQASM {
        return Err(Error::MissingVersion);
    }
    if version(tokens)? != 2.0 {
        return Err(Error::UnsupportedVersion);
    }

    while tokens.len() > 0 {
        let node = parse_node(tokens)?;
        nodes.push(node);
    }

    Ok(nodes)
}

fn version(tokens: &mut Vec<Token>) -> Result<f32> {
    let version = match_real(tokens)?;
    match_semicolon(tokens)?;

    Ok(version)
}

fn qreg(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // QReg -> Identifier -> Left Square Bracket -> Int -> Right Square Bracket -> Semicolon
    let identifier = match_identifier(tokens)?;

    match_token(tokens, Token::LSParen)?;

    let num = match_nninteger(tokens)?;
    match_token(tokens, Token::RSParen)?;
    match_semicolon(tokens)?;

    Ok(AstNode::QReg(identifier, num))
}

fn creg(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // CReg -> Identifier -> Left Square Bracket -> Int -> Right Square Bracket -> Semicolon
    let identifier = match_identifier(tokens)?;

    match_token(tokens, Token::LSParen)?;

    let num = match_nninteger(tokens)?;
    match_token(tokens, Token::RSParen)?;
    match_semicolon(tokens)?;

    Ok(AstNode::CReg(identifier, num))
}

fn if_(tokens: &mut Vec<Token>) -> Result<AstNode> {
    match_token(tokens, Token::LParen)?;
    let id = match_identifier(tokens)?;
    match_token(tokens, Token::Equals)?;
    let val = match_nninteger(tokens)?;
    match_token(tokens, Token::RParen)?;
    let node = parse_node(tokens)?;

    Ok(AstNode::If(id, val, Box::new(node)))
}

fn barrier(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // Barrier -> Argument -> Semicolon
    let argument = match_argument(tokens)?;
    match_semicolon(tokens)?;
    Ok(AstNode::Barrier(argument))
}

fn reset(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // reset -> Argument -> Semicolon
    let argument = match_argument(tokens)?;
    match_semicolon(tokens)?;
    Ok(AstNode::Reset(argument))
}

fn measure(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // Measure -> Argument -> Arrow -> Argument -> Semicolon
    let arg_1 = match_argument(tokens)?;
    match_token(tokens, Token::Arrow)?;
    let arg_2 = match_argument(tokens)?;
    match_semicolon(tokens)?;

    Ok(AstNode::Measure(arg_1, arg_2))
}

fn application(tokens: &mut Vec<Token>, id: String) -> Result<AstNode> {
    // id -> argument list -> Semicolon;
    // id -> () -> argument list -> Semicolon;
    // id -> ( Expr list ) ->
    let params = match match_token_peek(tokens, Token::LParen) {
        Ok(_) => {
            tokens.remove(0);
            match match_token_peek(tokens, Token::RParen) {
                Ok(_) => {
                    tokens.remove(0);
                    vec![]
                }
                Err(_) => {
                    let p = match_mathexpr_list(tokens)?;
                    match_token(tokens, Token::RParen)?;
                    p
                }
            }
        }
        Err(_) => vec![],
    };

    let list = match_argument_list(tokens)?;
    match_semicolon(tokens)?;
    Ok(AstNode::ApplyGate(id, list, params))
}

fn opaque(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // opaque -> id -> argument list -> Semicolon;
    // opaque -> id -> () -> argument list -> Semicolon;
    // opaque -> id -> ( Expr list ) ->
    let id = match_identifier(tokens)?;
    let params = match match_token_peek(tokens, Token::LParen) {
        Ok(_) => {
            tokens.remove(0);
            match match_token_peek(tokens, Token::RParen) {
                Ok(_) => {
                    tokens.remove(0);
                    vec![]
                }
                Err(_) => {
                    let p = match_id_list(tokens)?;
                    match_token(tokens, Token::RParen)?;
                    p
                }
            }
        }
        Err(_) => vec![],
    };

    let list = match_argument_list(tokens)?;
    match_semicolon(tokens)?;
    Ok(AstNode::Opaque(id, list, params))
}

fn gate(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // gate -> id -> argument list -> { -> list of applications -> }
    // gate -> id -> () -> argument list ->{ -> list of applications -> }
    // gate -> id -> ( Expr list ) -> { -> list of applications -> }
    let id = match_identifier(tokens)?;

    let params = match match_token_peek(tokens, Token::LParen) {
        Ok(_) => {
            tokens.remove(0);
            match match_token_peek(tokens, Token::RParen) {
                Ok(_) => {
                    tokens.remove(0);
                    vec![]
                }
                Err(_) => {
                    let p = match_id_list(tokens)?;
                    match_token(tokens, Token::RParen)?;
                    p
                }
            }
        }
        Err(_) => vec![],
    };

    let list = match_id_list(tokens)?;

    match_token(tokens, Token::LCParen)?;

    let applications = if tokens[0] != Token::LParen {
        match_application_list(tokens)?
    } else {
        vec![]
    };

    match_token(tokens, Token::RCParen)?;

    Ok(AstNode::Gate(id, list, params, applications))
}

//////////////////////////////////////////////////////////////////////
// Terminals
//////////////////////////////////////////////////////////////////////
fn match_application_list(tokens: &mut Vec<Token>) -> Result<Vec<AstNode>> {
    let mut args = vec![];
    let id = match_identifier(tokens)?;

    let head = application(tokens, id)?;

    args.push(head);

    match tokens[0] {
        Token::Id(_) => {
            let tail = match_application_list(tokens)?;
            for t in tail {
                args.push(t);
            }
        }
        _ => (),
    }

    Ok(args)
}

fn match_argument_list(tokens: &mut Vec<Token>) -> Result<Vec<Argument>> {
    let mut args = vec![];
    let head = match_argument(tokens)?;

    args.push(head);

    match match_token_peek(tokens, Token::Comma) {
        Ok(_) => {
            tokens.remove(0);
            let tail = match_argument_list(tokens)?;
            for t in tail {
                args.push(t);
            }
        }
        Err(_) => (),
    }

    Ok(args)
}

fn match_mathexpr_list(tokens: &mut Vec<Token>) -> Result<Vec<String>> {
    let mut args = vec![];
    let head = match_mathexpr(tokens)?;

    args.push(head);

    match match_token_peek(tokens, Token::Comma) {
        Ok(_) => {
            tokens.remove(0);
            let tail = match_mathexpr_list(tokens)?;
            for t in tail {
                args.push(t);
            }
        }
        Err(_) => (),
    }

    Ok(args)
}

fn match_id_list(tokens: &mut Vec<Token>) -> Result<Vec<String>> {
    let mut args = vec![];
    let head = match_identifier(tokens)?;

    args.push(head);

    match match_token_peek(tokens, Token::Comma) {
        Ok(_) => {
            tokens.remove(0);
            let tail = match_id_list(tokens)?;
            for t in tail {
                args.push(t);
            }
        }
        Err(_) => (),
    }

    Ok(args)
}

fn match_mathexpr(tokens: &mut Vec<Token>) -> Result<String> {
    if tokens.len() <= 0 {
        return Err(Error::SourceError);
    }

    let mut expr_string = String::from("");
    let mut num_open_paren = 0;

    // Parse until we find a comma, semicolon or a non matching paren
    while tokens.len() > 0 {
        let string: String = match tokens[0].clone() {
            Token::Real(f) => f.to_string(),
            Token::NNInteger(n) => n.to_string(),
            Token::Id(i) => i,
            Token::Pi => String::from("pi"),
            Token::LParen => {
                num_open_paren += 1;
                String::from("(")
            }
            Token::RParen => {
                if num_open_paren == 0 {
                    return Ok(expr_string);
                }
                num_open_paren -= 1;
                String::from(")")
            }
            Token::Plus => String::from("+"),
            Token::Minus => String::from("-"),
            Token::Times => String::from("*"),
            Token::Divide => String::from("/"),
            Token::Exp => String::from("^"),
            Token::Sin => String::from("sin"),
            Token::Cos => String::from("cos"),
            Token::Tan => String::from("tan"),
            Token::Ln => String::from("ln"),
            Token::Sqrt => String::from("sqrt"),
            _ => return Ok(expr_string),
        };

        tokens.remove(0);
        expr_string.push_str(" ");
        expr_string.push_str(&string);
        expr_string.push_str(" ");
    }

    println!("{}", expr_string);
    Ok(expr_string)
}

fn match_argument(tokens: &mut Vec<Token>) -> Result<Argument> {
    let id = match_identifier(tokens)?;

    match match_token_peek(tokens, Token::LSParen) {
        Ok(()) => {
            tokens.remove(0);
            let n = match_nninteger(tokens)?;
            match_token(tokens, Token::RSParen)?;
            Ok(Argument::Qubit(id, n))
        }
        Err(_) => Ok(Argument::Register(id)),
    }
}

fn match_real(tokens: &mut Vec<Token>) -> Result<f32> {
    if tokens.len() <= 0 {
        return Err(Error::SourceError);
    }
    match tokens.remove(0) {
        Token::Real(n) => Ok(n),
        _ => Err(Error::MissingReal),
    }
}

fn match_nninteger(tokens: &mut Vec<Token>) -> Result<i32> {
    if tokens.len() <= 0 {
        return Err(Error::SourceError);
    }
    match tokens.remove(0) {
        Token::NNInteger(n) => Ok(n),
        _ => Err(Error::MissingInt),
    }
}

fn match_identifier(tokens: &mut Vec<Token>) -> Result<String> {
    if tokens.len() <= 0 {
        return Err(Error::SourceError);
    }
    match tokens.remove(0) {
        Token::Id(s) => Ok(s),
        _ => Err(Error::MissingIdentifier),
    }
}

fn match_token(tokens: &mut Vec<Token>, token: Token) -> Result<()> {
    if tokens.len() <= 0 {
        return Err(Error::SourceError);
    }
    if tokens.remove(0) != token {
        return Err(Error::SourceError);
    }
    Ok(())
}

fn match_token_peek(tokens: &mut Vec<Token>, token: Token) -> Result<()> {
    if tokens.len() <= 0 {
        return Err(Error::SourceError);
    }
    if tokens[0] != token {
        return Err(Error::SourceError);
    }
    Ok(())
}

fn match_semicolon(tokens: &mut Vec<Token>) -> Result<()> {
    if tokens.len() <= 0 {
        return Err(Error::SourceError);
    }

    if tokens.remove(0) != Token::Semicolon {
        return Err(Error::MissingSemicolon);
    }
    Ok(())
}
