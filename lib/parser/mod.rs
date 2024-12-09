use std::slice::Iter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::ast::{BinOp, Expr, Ident, Literal, UnaryOp};

pub mod ast;

pub struct TokenCursor<'a> {
    tokens: Iter<'a, Token>,
}
impl<'a> TokenCursor {
    pub fn new(input: &'a Tokens) -> Self {
        Self {
            tokens: input.into_iter()
        }
    }
    pub fn peek_first(&self) -> Option<&Token> {
        self.tokens.clone().next()
    }
    pub fn next(&mut self) -> Option<&Token> {
        self.tokens.next()
    }
}
pub struct Parser<'a> {
    previous: Token,
    token_cursor: TokenCursor<'a>,
}

impl<'a> Parser {
    pub fn new(tokens: &'a Tokens) -> Self {
        Self {
            previous: Token::EOF,
            token_cursor: TokenCursor::new(tokens),
        }
    }
    fn advance(&mut self) {
        match self.token_cursor.next() {
            Some(tok) => { self.previous = tok.clone() },
            None => { self.previous = Token::EOF },
        }
    }

    fn peek(&self) -> Token {
        match self.token_cursor.peek_first() {
            Some(tok) => tok.clone(),
            None => Token::EOF,
        }
    }
    fn parse_expr(&mut self) -> Expr {
        self.parse_equality()
    }
    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();
        loop {
            match self.peek() {
                Token::Equal => {
                    self.advance();
                    let right = self.parse_comparison();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Equal, Box::from(right));
                }
                Token::NotEqual => {
                    self.advance();
                    let right = self.parse_comparison();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::NotEqual, Box::from(right));
                }
                _ => break
            }
        }
        expr
    }
    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();
        loop {
            match self.peek() {
                Token::Less => {
                    self.advance();
                    let right = self.parse_term();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Less, Box::from(right));
                }
                Token::LessEqual => {
                    self.advance();
                    let right = self.parse_term();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::LessEqual, Box::from(right));
                }
                Token::Greater => {
                    self.advance();
                    let right = self.parse_term();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Greater, Box::from(right));
                }
                Token::GreaterEqual => {
                    self.advance();
                    let right = self.parse_term();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::GreaterEqual, Box::from(right));
                }
                _ => break
            }
        }
        expr
    }
    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_factor();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Plus, Box::from(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_factor();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Minus, Box::from(right));
                }
                _ => break
            }
        }
        expr
    }
    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        loop {
            match self.peek() {
                Token::Multiply => {
                    self.advance();
                    let right = self.parse_unary();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Multiply, Box::from(right));
                }
                Token::Divide => {
                    self.advance();
                    let right = self.parse_unary();
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Divide, Box::from(right));
                }
                _ => break
            }
        }
        expr
    }
    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Token::Minus => {
                self.advance();
                Expr::UnaryExpr(UnaryOp::UnaryMinus, Box::from(self.parse_unary()))
            },
            Token::Plus => {
                self.advance();
                Expr::UnaryExpr(UnaryOp::UnaryPlus, Box::from(self.parse_unary()))
            },
            Token::Not => {
                self.advance();
                Expr::UnaryExpr(UnaryOp::Not, Box::from(self.parse_unary()))
            },
            _ => {
                self.parse_primary()
            },
        }
    }
    fn parse_primary(&mut self) -> Expr {
        match self.peek() {
            Token::Ident(ident) => {
                self.advance();
                Expr::IdentExpr(Ident(ident))
            },
            Token::BooleanLiteral(literal) => {
                self.advance();
                Expr::LiteralExpr(Literal::BoolLiteral(literal))
            },
            Token::IntLiteral(literal) => {
                self.advance();
                Expr::LiteralExpr(Literal::IntLiteral(literal))
            },
            Token::LParen => {
                self.advance();
                self.parse_expr()
            },
            _ => {
                self.parse_expr()
            }
            // todo: Need to handle syntax error!!!
        }
    }
}