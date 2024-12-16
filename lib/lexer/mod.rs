pub mod token;
mod cursor;

use crate::lexer::cursor::Cursor;
use crate::lexer::token::*;

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(input: &str) -> Tokens {
        let mut input_chars = Cursor::new(input);
        let mut tokens = vec![];
        loop {
            match input_chars.peek_first() {
                None => {
                    tokens.push(Self::lex_token(&mut input_chars));
                    break;
                }
                Some(c) if c.is_ascii_whitespace() => {
                    input_chars.next();
                }
                _ => {
                    tokens.push(Self::lex_token(&mut input_chars));
                }
            }
        }
        tokens
    }

    fn lex_token(input: &mut Cursor) -> Token {
        match input.next() {
            Some(';') => Token::SemiColon,
            Some(',') => Token::Comma,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Divide,
            Some('*') => Token::Multiply,
            Some('=') => {
                match input.peek_first() {
                    Some('=') => {
                        input.next();
                        Token::Equal
                    }
                    _ => Token::Assign
                }
            }
            Some('!') => {
                match input.peek_first() {
                    Some('=') => {
                        input.next();
                        Token::NotEqual
                    }
                    _ => Token::Not
                }
            }
            Some('<') => {
                match input.peek_first() {
                    Some('=') => {
                        input.next();
                        Token::LessEqual
                    }
                    _ => Token::Less
                }
            }
            Some('>') => {
                match input.peek_first() {
                    Some('=') => {
                        input.next();
                        Token::GreaterEqual
                    }
                    _ => Token::Greater
                }
            }
            Some('"') => {
                let mut s = String::new();
                loop {
                    match input.peek_first() {
                        Some('\n') | Some('\r') | None => {
                            input.next();
                            return Token::Illegal
                        }
                        Some('"') => {
                            input.next();
                            return Token::StringLiteral(s)
                        }
                        Some(ch) => {
                            input.next();
                            s.push(ch)
                        }
                    }
                }
            }
            Some(c) if c.is_alphabetic() || c.eq(&'_') => {
                let mut s = String::new();
                s.push(c);
                while let Some(ch) = input.peek_first() {
                    if ch.is_alphabetic() || ch.is_ascii_digit() || ch.eq(&'_') {
                        s.push(input.next().unwrap());
                    } else {
                        break
                    }
                }
                match s.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "true" => Token::BooleanLiteral(true),
                    "false" => Token::BooleanLiteral(false),
                    _ => Token::Ident(s)
                }
            }
            Some(c) if c.is_ascii_digit() => {
                let mut s = String::new();
                s.push(c);
                while let Some(ch) = input.peek_first() {
                    if ch.is_ascii_digit() {
                        s.push(input.next().unwrap());
                    } else {
                        break
                    }
                }
                match s.parse::<i64>() {
                    Ok(value) => Token::IntLiteral(value),
                    _ => Token::Illegal
                }
            }

            None => Token::EOF,
            _ => Token::Illegal
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer_1() {
        let s = String::from("if (a == 10) {\
                return a;\
             } else if (a != 20) {\
                return !a;\
            } else if (a > 20) {\
                return -30 / 40 * 50;\
            } else if (a < 30) {\
                return true;\
            }\
            let x = \"hello world!\"\
            return false;\
            ");
        let result = Lexer::lex_tokens(s.as_str());
        let expected_result = vec![
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::Equal,
            Token::IntLiteral(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::NotEqual,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Not,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::Greater,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Minus,
            Token::IntLiteral(30),
            Token::Divide,
            Token::IntLiteral(40),
            Token::Multiply,
            Token::IntLiteral(50),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::Less,
            Token::IntLiteral(30),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::BooleanLiteral(true),
            Token::SemiColon,
            Token::RBrace,
            Token::Let,
            Token::Ident("x".to_owned()),
            Token::Assign,
            Token::StringLiteral("hello world!".to_owned()),
            Token::Return,
            Token::BooleanLiteral(false),
            Token::SemiColon,
            Token::EOF,
        ];
        assert_eq!(result, expected_result);
    }
}