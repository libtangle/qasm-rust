//! This module implements the various parsing methods.
//! Most methods are not documented, and should only be accessed
//! indirectly from the `parse` method.

use token::Token;
use error::Error;
use ast::{Argument, AstNode};
use std::result;

type Result<T> = result::Result<T, Error>;

pub fn parse_node(tokens: &mut Vec<Token>) -> Result<AstNode> {
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

        _ => Err(Error::SourceError),
    }
}

pub fn version(tokens: &mut Vec<Token>) -> Result<f32> {
    let version = match_real(tokens)?;
    match_semicolon(tokens)?;

    Ok(version)
}

pub fn qreg(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // QReg -> Identifier -> Left Square Bracket -> Int -> Right Square Bracket -> Semicolon
    let identifier = match_identifier(tokens)?;

    match_token(tokens, Token::LSParen)?;

    let num = match_nninteger(tokens)?;
    match_token(tokens, Token::RSParen)?;
    match_semicolon(tokens)?;

    Ok(AstNode::QReg(identifier, num))
}

pub fn creg(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // CReg -> Identifier -> Left Square Bracket -> Int -> Right Square Bracket -> Semicolon
    let identifier = match_identifier(tokens)?;

    match_token(tokens, Token::LSParen)?;

    let num = match_nninteger(tokens)?;
    match_token(tokens, Token::RSParen)?;
    match_semicolon(tokens)?;

    Ok(AstNode::CReg(identifier, num))
}

pub fn if_(tokens: &mut Vec<Token>) -> Result<AstNode> {
    match_token(tokens, Token::LParen)?;
    let id = match_identifier(tokens)?;
    match_token(tokens, Token::Equals)?;
    let val = match_nninteger(tokens)?;
    match_token(tokens, Token::RParen)?;
    let node = parse_node(tokens)?;

    Ok(AstNode::If(id, val, Box::new(node)))
}

pub fn barrier(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // Barrier -> Argument -> Semicolon
    let argument = match_argument(tokens)?;
    match_semicolon(tokens)?;
    Ok(AstNode::Barrier(argument))
}

pub fn reset(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // reset -> Argument -> Semicolon
    let argument = match_argument(tokens)?;
    match_semicolon(tokens)?;
    Ok(AstNode::Reset(argument))
}

pub fn measure(tokens: &mut Vec<Token>) -> Result<AstNode> {
    // Measure -> Argument -> Arrow -> Argument -> Semicolon
    let arg_1 = match_argument(tokens)?;
    match_token(tokens, Token::Arrow)?;
    let arg_2 = match_argument(tokens)?;
    match_semicolon(tokens)?;

    Ok(AstNode::Measure(arg_1, arg_2))
}

pub fn application(tokens: &mut Vec<Token>, id: String) -> Result<AstNode> {
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

pub fn opaque(tokens: &mut Vec<Token>) -> Result<AstNode> {
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

pub fn gate(tokens: &mut Vec<Token>) -> Result<AstNode> {
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

    let applications = if tokens[0] != Token::RCParen {
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
pub fn match_application_list(tokens: &mut Vec<Token>) -> Result<Vec<AstNode>> {
    let mut args = vec![];
    let id = match_identifier(tokens)?;

    let head = application(tokens, id)?;

    args.push(head);

    if let Token::Id(_) = tokens[0] {
        let tail = match_application_list(tokens)?;
        for t in tail {
            args.push(t);
        }
    }
    Ok(args)
}

pub fn match_argument_list(tokens: &mut Vec<Token>) -> Result<Vec<Argument>> {
    let mut args = vec![];
    let head = match_argument(tokens)?;

    args.push(head);

    if match_token_peek(tokens, Token::Comma).is_ok() {
        tokens.remove(0);
        let tail = match_argument_list(tokens)?;
        for t in tail {
            args.push(t);
        }
    }

    Ok(args)
}

pub fn match_mathexpr_list(tokens: &mut Vec<Token>) -> Result<Vec<String>> {
    let mut args = vec![];
    let head = match_mathexpr(tokens)?;

    args.push(head);

    if match_token_peek(tokens, Token::Comma).is_ok() {
        tokens.remove(0);
        let tail = match_mathexpr_list(tokens)?;
        for t in tail {
            args.push(t);
        }
    }

    Ok(args)
}

pub fn match_id_list(tokens: &mut Vec<Token>) -> Result<Vec<String>> {
    let mut args = vec![];
    let head = match_identifier(tokens)?;

    args.push(head);

    if match_token_peek(tokens, Token::Comma).is_ok() {
        tokens.remove(0);
        let tail = match_id_list(tokens)?;
        for t in tail {
            args.push(t);
        }
    }

    Ok(args)
}

pub fn match_mathexpr(tokens: &mut Vec<Token>) -> Result<String> {
    if tokens.is_empty() {
        return Err(Error::SourceError);
    }

    let mut expr_string = String::from("");
    let mut num_open_paren = 0;

    // Parse until we find a comma, semicolon or a non matching paren
    while !tokens.is_empty() {
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

    Ok(expr_string)
}

pub fn match_argument(tokens: &mut Vec<Token>) -> Result<Argument> {
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

pub fn match_real(tokens: &mut Vec<Token>) -> Result<f32> {
    if tokens.is_empty() {
        return Err(Error::SourceError);
    }
    match tokens.remove(0) {
        Token::Real(n) => Ok(n),
        _ => Err(Error::MissingReal),
    }
}

pub fn match_nninteger(tokens: &mut Vec<Token>) -> Result<i32> {
    if tokens.is_empty() {
        return Err(Error::SourceError);
    }
    match tokens.remove(0) {
        Token::NNInteger(n) => Ok(n),
        _ => Err(Error::MissingInt),
    }
}

pub fn match_identifier(tokens: &mut Vec<Token>) -> Result<String> {
    if tokens.is_empty() {
        return Err(Error::SourceError);
    }
    match tokens.remove(0) {
        Token::Id(s) => Ok(s),
        _ => Err(Error::MissingIdentifier),
    }
}

pub fn match_token(tokens: &mut Vec<Token>, token: Token) -> Result<()> {
    if tokens.is_empty() {
        return Err(Error::SourceError);
    }
    if tokens.remove(0) != token {
        return Err(Error::SourceError);
    }
    Ok(())
}

pub fn match_token_peek(tokens: &mut Vec<Token>, token: Token) -> Result<()> {
    if tokens.is_empty() {
        return Err(Error::SourceError);
    }
    if tokens[0] != token {
        return Err(Error::SourceError);
    }
    Ok(())
}

pub fn match_semicolon(tokens: &mut Vec<Token>) -> Result<()> {
    if tokens.is_empty() {
        return Err(Error::SourceError);
    }

    if tokens.remove(0) != Token::Semicolon {
        return Err(Error::MissingSemicolon);
    }
    Ok(())
}
