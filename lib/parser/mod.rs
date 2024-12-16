use std::slice::Iter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::ast::{BinOp, Expr, Ident, Literal, Program, Stmt, UnaryOp};

pub mod ast;

pub struct TokenCursor<'a> {
    tokens: Iter<'a, Token>,
}
pub struct ParseError {
    token: Token,
    message: String,
}
impl<'a> TokenCursor<'a> {
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
    errors: Vec<ParseError>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens) -> Self {
        Self {
            previous: Token::EOF,
            token_cursor: TokenCursor::new(tokens),
            errors: vec![],
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
    fn error(&mut self, message: &str, token: Token) {
        self.errors.push(
            ParseError {
                token,
                message: message.to_string(),
            }
        )
    }
    fn synchronize(&mut self) {
        loop {
            match self.peek() {
                Token::Let | Token::If | Token::Return | Token::Function | Token::EOF => {
                    return;
                }
                Token::SemiColon => {
                    self.advance();
                    return;
                }
                _ => self.advance()
            }
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = vec![];
        while self.peek() != Token::EOF {
            match self.parse_stmt() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(_) => {
                    self.synchronize()
                }
            }
        }
        statements
    }
    fn parse_stmt(&mut self) -> Result<Stmt, ()> {
        match self.peek() {
            Token::Let => {
                self.advance();
                self.parse_let()
            }
            Token::LBrace => {
                self.advance();
                self.parse_block()
            }
            Token::If => {
                self.advance();
                self.parse_if()
            }
            Token::While => {
                self.advance();
                self.parse_while()
            }
            _ => {
                self.parse_expr_stmt()
            }
        }
    }
    fn parse_while(&mut self) -> Result<Stmt, ()> {
        let condition = self.parse_expr()?;
        if self.peek() != Token::LBrace {
            self.error("Expect '{' after while condition", self.previous.clone());
            return Err(())
        }
        self.advance();
        let loop_block = self.parse_block()?;
        Ok(Stmt::WhileStmt(condition, Box::from(loop_block)))
    }
    fn parse_if(&mut self) -> Result<Stmt, ()> {
        let condition = self.parse_expr()?;
        if self.peek() != Token::LBrace {
            self.error("Expect '{' after if condition", self.previous.clone());
            return Err(())
        }
        self.advance();
        let then_branch = self.parse_block()?;
        if self.peek() == Token::Else {
            self.advance();
            if self.peek() == Token::If {
                self.advance();
                let else_branch = self.parse_if()?;
                Ok(Stmt::IfStmt(condition, Box::from(then_branch), Some(Box::from(else_branch))))
            } else {
                let else_branch = self.parse_block()?;
                Ok(Stmt::IfStmt(condition, Box::from(then_branch), Some(Box::from(else_branch))))
            }
        } else {
            Ok(Stmt::IfStmt(condition, Box::from(then_branch), None))
        }
    }
    fn parse_block(&mut self) -> Result<Stmt, ()> {
        let mut statements = vec![];
        loop {
            match self.peek() {
                Token::RBrace | Token::EOF => {
                    break;
                },
                _ => {
                    let stmt_res = self.parse_stmt();
                    if let Ok(stmt) = stmt_res {
                        statements.push(stmt);
                    }
                }
            }
        }
        if self.peek() == Token::RBrace {
            self.advance();
            Ok(Stmt::BlockStmt(statements))
        } else{
            self.error("Expect '}' after block", self.previous.clone());
            Err(())
        }
    }
    fn parse_let(&mut self) -> Result<Stmt, ()> {
        if let Token::Ident(ident) = self.peek() {
            self.advance();
            match self.peek() {
                Token::Assign => {
                    let expr = self.parse_expr()?;
                    match self.peek() {
                        Token::SemiColon => {
                            self.advance();
                            Ok(Stmt::LetStmt(Ident(ident), expr))
                        }
                        _ => {
                            self.error("Expected ';' after statement", self.previous.clone());
                            Err(())
                        }
                    }
                }
                Token::SemiColon => {
                    self.advance();
                    Ok(Stmt::LetStmt(Ident(ident.clone()), Expr::IdentExpr(Ident(ident))))
                }
                _ => {
                    self.error("Expected ';' after statement", self.previous.clone());
                    Err(())
                }
            }
        } else {
            self.error("Expect identifier after 'let'", self.previous.clone());
            Err(())
        }
    }
    fn parse_expr_stmt(&mut self) -> Result<Stmt, ()> {
        let expr = self.parse_expr()?;
        match self.peek() {
            Token::SemiColon => {
                self.advance();
                Ok(Stmt::ExprStmt(expr))
            },
            _ => {
                self.error("Expected ';' after expression", self.previous.clone());
                Err(())
            }
        }
    }
    fn parse_expr(&mut self) -> Result<Expr, ()> {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Result<Expr, ()> {
        let left = self.parse_equality()?;
        match self.peek() {
            Token::Assign => {
                self.advance();
                let expr = self.parse_equality()?;
                if let Expr::IdentExpr(ident) = left {
                    Ok(Expr::AssignmentExpr(ident, Box::from(expr)))
                } else {
                    self.error("Illegal assignment", self.previous.clone());
                    Err(())
                }
            }
            _ => Ok(left)
        }
    }
    fn parse_equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_comparison()?;
        loop {
            match self.peek() {
                Token::Equal => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Equal, Box::from(right));
                }
                Token::NotEqual => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::NotEqual, Box::from(right));
                }
                _ => break
            }
        }
        Ok(expr)
    }
    fn parse_comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_term()?;
        loop {
            match self.peek() {
                Token::Less => {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Less, Box::from(right));
                }
                Token::LessEqual => {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::LessEqual, Box::from(right));
                }
                Token::Greater => {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Greater, Box::from(right));
                }
                Token::GreaterEqual => {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::GreaterEqual, Box::from(right));
                }
                _ => break
            }
        }
        Ok(expr)
    }
    fn parse_term(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_factor()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Plus, Box::from(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Minus, Box::from(right));
                }
                _ => break
            }
        }
        Ok(expr)
    }
    fn parse_factor(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_unary()?;
        loop {
            match self.peek() {
                Token::Multiply => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Multiply, Box::from(right));
                }
                Token::Divide => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = Expr::BinExpr(Box::from(expr), BinOp::Divide, Box::from(right));
                }
                _ => break
            }
        }
        Ok(expr)
    }
    fn parse_unary(&mut self) -> Result<Expr, ()> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryExpr(UnaryOp::UnaryMinus, Box::from(expr)))
            },
            Token::Plus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryExpr(UnaryOp::UnaryPlus, Box::from(expr)))
            },
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryExpr(UnaryOp::Not, Box::from(expr)))
            },
            _ => {
                self.parse_primary()
            },
        }
    }
    fn parse_primary(&mut self) -> Result<Expr, ()> {
        match self.peek() {
            Token::Ident(ident) => {
                self.advance();
                Ok(Expr::IdentExpr(Ident(ident)))
            },
            Token::BooleanLiteral(literal) => {
                self.advance();
                Ok(Expr::LiteralExpr(Literal::BoolLiteral(literal)))
            },
            Token::IntLiteral(literal) => {
                self.advance();
                Ok(Expr::LiteralExpr(Literal::IntLiteral(literal)))
            },
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                match self.peek() {
                    Token::RParen => {
                        self.advance();
                        Ok(expr)
                    },
                    _ => {
                        self.error("Expected ')' after expression", self.previous.clone());
                        Err(())
                    },
                }
            },
            tok => {
                self.error("Unexpected Token", tok.clone());
                Err(())
            }
        }
    }
}