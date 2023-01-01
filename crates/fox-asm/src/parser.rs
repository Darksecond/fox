use crate::tokenizer::Token;
use fox_bytecode::Opcode;

#[derive(Debug)]
pub enum Stmt {
    OriginAbsolute(u32),
    OriginRelative(u32),
    LiteralWord(u32),
    LabelAbsolute(String),
    ReferenceAbsolute(String),
    Operation(Opcode),
    String(String),
    RawByte(u8),
    RawWord(u32),
    RawReferenceAbsolute(String),
}

pub type Ast = Vec<Stmt>;

fn parse_identifier<'a>(it: &mut impl Iterator<Item=&'a Token>) -> &'a str {
    if let Some(Token::IdentifierOrNumber(str)) = it.next() {
        return str;
    } else {
        panic!("Invalid value");
    }
}

fn parse_number<'a>(it: &mut impl Iterator<Item=&'a Token>) -> u32 {
    let str = parse_identifier(it);
    u32::from_str_radix(&str, 16).unwrap()
}

pub fn parse(tokens: &[Token]) -> Ast {
    let mut ast = Vec::new();
    let mut it = tokens.iter();

    while let Some(token) = it.next() {
        match token {
            Token::At => {
                let str = parse_identifier(&mut it);
                ast.push(Stmt::LabelAbsolute(str.to_string()));
            },
            Token::Semicolon => {
                let str = parse_identifier(&mut it);
                ast.push(Stmt::ReferenceAbsolute(str.to_string()));
            },
            Token::Colon => {
                let str = parse_identifier(&mut it);
                ast.push(Stmt::RawReferenceAbsolute(str.to_string()));
            },
            Token::Pound => {
                let number = parse_number(&mut it);
                ast.push(Stmt::LiteralWord(number));
            },
            Token::Pipe => {
                let number = parse_number(&mut it);
                ast.push(Stmt::OriginAbsolute(number));
            },
            Token::Ampersand => todo!(),
            Token::Period => {
                let number = parse_number(&mut it);
                ast.push(Stmt::RawByte(number as _));
            },
            Token::Equal => {
                let number = parse_number(&mut it);
                ast.push(Stmt::RawWord(number));
            },
            Token::IdentifierOrNumber(str) => {
                use std::str::FromStr;

                //TODO rework this error handling here
                let op = Opcode::from_str(str).expect(&format!("Could not parse {}", str));
                ast.push(Stmt::Operation(op));
            },
            Token::String(value) => {
                ast.push(Stmt::String(value.to_string()));
            },
            Token::Dollar => {
                let number = parse_number(&mut it);
                ast.push(Stmt::OriginRelative(number));
            },
            Token::UnterminatedString => todo!(),
            Token::Unknown(x) => todo!("{}", x),
        }
    }

    ast
}
