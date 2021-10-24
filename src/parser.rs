//! This module implements the various parsing methods.
//! Most methods are not documented, and should only be accessed
//! indirectly from the `parse` method.

use std::iter::Peekable;
use token::Token;
use error::Error;
use ast::{Argument, AstNode};
use std::result;

const SUPPORTED_VERSIONS: [f32; 1] = [
    2.0,
];

type TokenStream<'a> = Peekable<std::slice::Iter<'a, Token>>;
type Result<T> = result::Result<T, Error>;

pub fn parse(tokens: &mut TokenStream) -> Result<Vec<AstNode>> {
    let mut nodes = vec![];

    // Check that the version is first, and that it is version 2.0
    if !SUPPORTED_VERSIONS.contains(&version(tokens)?) {
        return Err(Error::UnsupportedVersion);
    }

    while let Some(token) = tokens.peek() {
        let node = parse_node(tokens)?;
        nodes.push(node);
    }

    Ok(nodes)
}

fn parse_node(tokens: &mut TokenStream) -> Result<AstNode> {
    match tokens.next().ok_or(Error::SourceError)?.clone() {
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

pub fn version(tokens: &mut TokenStream) -> Result<f32> {
    match_token(tokens, Token::OpenQASM)
        .map_err(|_| Error::MissingVersion)?;
    let version = match_real(tokens)?;
    match_semicolon(tokens)?;

    Ok(version)
}

pub fn qreg(tokens: &mut TokenStream) -> Result<AstNode> {
    // QReg -> Identifier -> Left Square Bracket -> Int -> Right Square Bracket -> Semicolon
    let identifier = match_identifier(tokens)?;
    match_token(tokens, Token::LSParen)?;
    let num = match_nninteger(tokens)?;
    match_token(tokens, Token::RSParen)?;
    match_semicolon(tokens)?;

    Ok(AstNode::QReg(identifier, num))
}

pub fn creg(tokens: &mut TokenStream) -> Result<AstNode> {
    // CReg -> Identifier -> Left Square Bracket -> Int -> Right Square Bracket -> Semicolon
    let identifier = match_identifier(tokens)?;
    match_token(tokens, Token::LSParen)?;
    let num = match_nninteger(tokens)?;
    match_token(tokens, Token::RSParen)?;
    match_semicolon(tokens)?;

    Ok(AstNode::CReg(identifier, num))
}

pub fn if_(tokens: &mut TokenStream) -> Result<AstNode> {
    match_token(tokens, Token::LParen)?;
    let id = match_identifier(tokens)?;
    match_token(tokens, Token::Equals)?;
    let val = match_nninteger(tokens)?;
    match_token(tokens, Token::RParen)?;
    let node = parse_node(tokens)?;

    Ok(AstNode::If(id, val, Box::new(node)))
}

pub fn barrier(tokens: &mut TokenStream) -> Result<AstNode> {
    // Barrier -> Argument -> Semicolon
    let argument = match_argument(tokens)?;
    match_semicolon(tokens)?;

    Ok(AstNode::Barrier(argument))
}

pub fn reset(tokens: &mut TokenStream) -> Result<AstNode> {
    // reset -> Argument -> Semicolon
    let argument = match_argument(tokens)?;
    match_semicolon(tokens)?;

    Ok(AstNode::Reset(argument))
}

pub fn measure(tokens: &mut TokenStream) -> Result<AstNode> {
    // Measure -> Argument -> Arrow -> Argument -> Semicolon
    let arg_1 = match_argument(tokens)?;
    match_token(tokens, Token::Arrow)?;
    let arg_2 = match_argument(tokens)?;
    match_semicolon(tokens)?;

    Ok(AstNode::Measure(arg_1, arg_2))
}

pub fn application(tokens: &mut TokenStream, id: String) -> Result<AstNode> {
    // id -> argument list -> Semicolon;
    // id -> () -> argument list -> Semicolon;
    // id -> ( Expr list ) ->
    let params = if let Some(Token::LParen) = tokens.peek() {
        tokens.next();
        if let Some(Token::RParen) = tokens.peek() {
            tokens.next();
            vec![]
        } else {
            let p = match_mathexpr_list(tokens)?;
            match_token(tokens, Token::RParen)?;
            p
        }
    } else {
        vec![]
    };

    let list = match_argument_list(tokens)?;
    match_semicolon(tokens)?;

    Ok(AstNode::ApplyGate(id, list, params))
}

pub fn opaque(tokens: &mut TokenStream) -> Result<AstNode> {
    // opaque -> id -> argument list -> Semicolon;
    // opaque -> id -> () -> argument list -> Semicolon;
    // opaque -> id -> ( Expr list ) ->
    let id = match_identifier(tokens)?;

    let params = if let Some(Token::LParen) = tokens.peek() {
        tokens.next();
        if let Some(Token::RParen) = tokens.peek() {
            tokens.next();
            vec![]
        } else {
            let p = match_id_list(tokens)?;
            match_token(tokens, Token::RParen)?;
            p
        }
    } else {
        vec![]
    };

    let list = match_argument_list(tokens)?;
    match_semicolon(tokens)?;

    Ok(AstNode::Opaque(id, list, params))
}

pub fn gate(tokens: &mut TokenStream) -> Result<AstNode> {
    // gate -> id -> argument list -> { -> list of applications -> }
    // gate -> id -> () -> argument list ->{ -> list of applications -> }
    // gate -> id -> ( Expr list ) -> { -> list of applications -> }
    let id = match_identifier(tokens)?;

    let params = if let Some(Token::LParen) = tokens.peek() {
        tokens.next();
        if let Some(Token::RParen) = tokens.peek() {
            tokens.next();
            vec![]
        } else {
            let p = match_id_list(tokens)?;
            match_token(tokens, Token::RParen)?;
            p
        }
    } else {
        vec![]
    };

    let list = match_id_list(tokens)?;
    match_token(tokens, Token::LCParen)?;

    let applications = if tokens.peek().ok_or(Error::SourceError)? != &&Token::RCParen {
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
pub fn match_application_list(tokens: &mut TokenStream) -> Result<Vec<AstNode>> {
    let id = match_identifier(tokens)?;
    let head = application(tokens, id)?;
    let mut args = vec![head];

    while let &Token::Id(id) = tokens.peek().ok_or(Error::SourceError)? {
        let tail = application(tokens, id.clone())?;
        args.push(tail);
    }

    Ok(args)
}

pub fn match_argument_list(tokens: &mut TokenStream) -> Result<Vec<Argument>> {
    let head = match_argument(tokens)?;
    let mut args = vec![head];

    while let Some(Token::Comma) = tokens.peek() {
        tokens.next();
        let tail = match_argument(tokens)?;
        args.push(tail);
    }

    Ok(args)
}

pub fn match_mathexpr_list(tokens: &mut TokenStream) -> Result<Vec<String>> {
    let head = match_mathexpr(tokens)?;
    let mut args = vec![head];

    while let Some(Token::Comma) = tokens.peek() {
        tokens.next();
        let tail = match_mathexpr(tokens)?;
        args.push(tail);
    }

    Ok(args)
}

pub fn match_id_list(tokens: &mut TokenStream) -> Result<Vec<String>> {
    let head = match_identifier(tokens)?;
    let mut args = vec![head];

    while let Some(Token::Comma) = tokens.peek() {
        tokens.next();
        let tail = match_identifier(tokens)?;
        args.push(tail);
    }

    Ok(args)
}

pub fn match_mathexpr(tokens: &mut TokenStream) -> Result<String> {
    if let None = tokens.peek() {
        return Err(Error::SourceError);
    }

    let mut expr_string = String::from("");
    let mut num_open_paren = 0;

    // Parse until we find a comma, semicolon or a non matching paren
    while let Some(token) = tokens.peek().cloned() {
        let string: String = match token.clone() {
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

        tokens.next();
        expr_string.push_str(" ");
        expr_string.push_str(&string);
        expr_string.push_str(" ");
    }

    Ok(expr_string)
}

pub fn match_argument(tokens: &mut TokenStream) -> Result<Argument> {
    let id = match_identifier(tokens)?;

    if let Some(Token::LSParen) = tokens.peek() {
        tokens.next();
        let n = match_nninteger(tokens)?;
        match_token(tokens, Token::RSParen)?;
        Ok(Argument::Qubit(id, n))
    } else {
        Ok(Argument::Register(id))
    }
}

pub fn match_real(tokens: &mut TokenStream) -> Result<f32> {
    match tokens.next() {
        Some(Token::Real(n)) => Ok(*n),
        Some(_) => Err(Error::MissingReal),
        None => Err(Error::SourceError),
    }
}

pub fn match_nninteger(tokens: &mut TokenStream) -> Result<i32> {
    match tokens.next() {
        Some(Token::NNInteger(n)) => Ok(*n),
        Some(_) => Err(Error::MissingInt),
        None => Err(Error::SourceError),
    }
}

pub fn match_identifier(tokens: &mut TokenStream) -> Result<String> {
    if let None = tokens.peek() {
        return Err(Error::SourceError);
    }
    match tokens.next() {
        Some(Token::Id(s)) => Ok(s.clone()),
        Some(_) => Err(Error::MissingIdentifier),
        None => Err(Error::SourceError),
    }
}

pub fn match_token(tokens: &mut TokenStream, eq_token: Token) -> Result<()> {
    match tokens.next() {
        Some(token) if &eq_token == token => Ok(()),
        _ => Err(Error::SourceError),
    }
}

#[allow(dead_code)]
pub fn match_token_peek(tokens: &mut TokenStream, eq_token: Token) -> Result<()> {
    match tokens.peek().copied() {
        Some(token) if &eq_token == token => Ok(()),
        _ => Err(Error::SourceError),
    }
}

pub fn match_semicolon(tokens: &mut TokenStream) -> Result<()> {
    match tokens.next() {
        Some(&Token::Semicolon) => Ok(()),
        Some(_) => Err(Error::MissingSemicolon),
        _ => Err(Error::SourceError),
    }
}
