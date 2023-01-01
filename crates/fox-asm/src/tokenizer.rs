use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    At,
    Semicolon,
    Pound,
    Pipe,
    Ampersand,
    Period,
    Dollar,
    Equal,
    Colon,

    IdentifierOrNumber(String),
    String(String),

    UnterminatedString,
    Unknown(char),
}

struct Scanner<'a> {
    it: Peekable<Chars<'a>>
}

impl<'a> Scanner<'a> {
    fn new(buf: &str) -> Scanner {
        Scanner {
            it: buf.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.it.next()
    }

    fn consume_while<F>(&mut self, x: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek() {
            if x(ch) {
                self.next().unwrap();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }
}

struct Lexer<'a> {
    it: Scanner<'a>,
}

impl<'a> Lexer<'a> {
    fn new(buf: &str) -> Lexer {
        Lexer {
            it: Scanner::new(buf),
        }
    }

    fn match_token(&mut self, ch: char) -> Option<Token> {
        match ch {
            '|' => Some(Token::Pipe),
            '@' => Some(Token::At),
            '&' => Some(Token::Ampersand),
            ';' => Some(Token::Semicolon),
            '#' => Some(Token::Pound),
            '$' => Some(Token::Dollar),
            '=' => Some(Token::Equal),
            ':' => Some(Token::Colon),
            '.' => Some(Token::Period),
            ' ' => None,
            '\n' => None,
            '\r' => None,
            '\t' => None,
            '(' => {
                let mut level = 1;
                while let Some(ch) = self.it.next() {
                    match ch {
                        '(' => level += 1,
                        ')' => {
                            level -= 1;
                            if level == 0 {
                                break;
                            }
                        },
                        _ => (),
                    }
                }
                None
            },
            '"' => {
                let string: String = self.it.consume_while(|ch| ch != '"').into_iter().collect();
                // Skip last "
                match self.it.next() {
                    None => Some(Token::UnterminatedString),
                    _ => Some(Token::String(string)),
                }
            }
            x if x.is_ascii_alphanumeric() => self.identifier(Some(x)),
            c => Some(Token::Unknown(c)),
        }
    }

    fn identifier(&mut self, ch: Option<char>) -> Option<Token> {
        let mut identifier = String::new();
        if let Some(ch) = ch {
            identifier.push(ch);
        }

        let rest: String = self
            .it
            .consume_while(|a| a.is_ascii_alphanumeric() || a == '_' || a == '-' || a == '/')
            .into_iter()
            .collect();
        identifier.push_str(rest.as_str());

        Some(Token::IdentifierOrNumber(identifier))
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let ch = match self.it.next() {
                None => break,
                Some(c) => c,
            };

            if let Some(token) = self.match_token(ch) {
                tokens.push(token);
            }
        }

        tokens
    }
}

pub fn tokenize(buf: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(buf);
    lexer.tokenize()
}
