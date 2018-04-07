use token;
use token::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn peek_is_alphanumeric(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_alphanumeric(ch),
            None => false,
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_alphanumeric() {
            ident.push(self.read_char().unwrap());
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() && c != '.' {
                break;
            }
            number.push(self.read_char().unwrap());
        }

        number
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equals
                } else {
                    // Shouldn't Be A Single Equals!
                    panic!("Error: Missing Charachter. Expected another `=`.");
                }
            }
            Some('+') => Token::Plus,
            Some('-') => {
                if self.peek_char_eq('>') {
                    self.read_char();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            Some('*') => Token::Times,
            Some('/') => Token::Divide,
            Some('^') => Token::Power,
            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('(') => Token::LParen,
            Some('[') => Token::LSParen,
            Some('{') => Token::LCParen,
            Some(')') => Token::RParen,
            Some(']') => Token::RSParen,
            Some('}') => Token::RCParen,
            Some(ch) => {
                if is_letter(ch) {
                    let literal = self.read_identifier(ch);
                    token::lookup_ident(&literal)
                } else if ch.is_numeric() {
                    let num_str = self.read_number(ch);
                    if num_str.contains('.') {
                        let num = num_str.parse::<f32>().unwrap();
                        Token::Real(num)
                    } else {
                        let num = num_str.parse::<i32>().unwrap();
                        Token::NNInteger(num)
                    }
                } else {
                    Token::Illegal
                }
            }

            // EOF
            None => Token::EndOfFile,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let tok = self.next_token();
        if tok == Token::EndOfFile {
            None
        } else {
            Some(tok)
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
fn is_alphanumeric(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

#[test]
fn is_letter_test() {
    assert!(is_letter('_'));
    assert!(is_letter('a'));
    assert!(is_letter('Z'));

    assert!(!is_letter('*'));
    assert!(!is_letter('1'));
}
