use std::slice::Iter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::ast::{BinOp, Expr, Ident, Literal, Program, Stmt, UnaryOp};

pub mod ast;

pub struct TokenCursor<'a> {
    tokens: Iter<'a, Token>,
}
#[derive(Debug)]
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
    fn error(&mut self, message: &str) {
        self.errors.push(
            ParseError {
                token: self.previous.clone(),
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
            Token::Function => {
                self.advance();
                self.parse_fn()
            }
            Token::Return => {
                self.advance();
                self.parse_return()
            }
            _ => {
                self.parse_expr_stmt()
            }
        }
    }
    fn parse_return(&mut self) -> Result<Stmt, ()> {
        if self.peek() == Token::SemiColon {
            self.advance();
            Ok(Stmt::ReturnStmt(None))
        } else {
            let expr = self.parse_expr()?;
            if self.peek() != Token::SemiColon {
                self.error("Expected ';' after statement");
                Err(())
            } else {
                self.advance();
                Ok(Stmt::ReturnStmt(Some(expr)))
            }
        }
    }
    fn parse_fn(&mut self) -> Result<Stmt, ()> {
        if let Token::Ident(ident) = self.peek() {
            self.advance();
            if self.peek() != Token::LParen {
                self.error("Expected '(' after identifier");
                return Err(())
            }
            self.advance();
            let mut parameters = vec![];
            if self.peek() != Token::RParen {
                parameters.push(self.parse_expr()?);
                while self.peek() == Token::Comma {
                    self.advance();
                    parameters.push(self.parse_expr()?);
                    if parameters.len() > 255 {
                        self.error("Cannot have more than 255 parameters");
                        return Err(())
                    }
                }
            }
            if self.peek() != Token::RParen {
                self.error("Expected ')' after parameters");
                return Err(())
            }
            self.advance();
            if self.peek() != Token::LBrace {
                self.error("Expected '{' after parameters");
            }
            self.advance();
            let body = self.parse_block()?;
            Ok(Stmt::FnStmt(Ident(ident), parameters, Box::from(body)))
        } else {
            self.error("Expected identifier after function definition");
            Err(())
        }
    }
    fn parse_while(&mut self) -> Result<Stmt, ()> {
        let condition = self.parse_expr()?;
        if self.peek() != Token::LBrace {
            self.error("Expect '{' after while condition");
            return Err(())
        }
        self.advance();
        let loop_block = self.parse_block()?;
        Ok(Stmt::WhileStmt(condition, Box::from(loop_block)))
    }
    fn parse_if(&mut self) -> Result<Stmt, ()> {
        let condition = self.parse_expr()?;
        if self.peek() != Token::LBrace {
            self.error("Expect '{' after if condition");
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
            self.error("Expect '}' after block");
            Err(())
        }
    }
    fn parse_let(&mut self) -> Result<Stmt, ()> {
        if let Token::Ident(ident) = self.peek() {
            self.advance();
            match self.peek() {
                Token::Assign => {
                    self.advance();
                    let expr = self.parse_expr()?;
                    match self.peek() {
                        Token::SemiColon => {
                            self.advance();
                            Ok(Stmt::LetStmt(Ident(ident), Some(expr)))
                        }
                        _ => {
                            self.error("Expected ';' after statement");
                            Err(())
                        }
                    }
                }
                Token::SemiColon => {
                    self.advance();
                    Ok(Stmt::LetStmt(Ident(ident.clone()), None))
                }
                _ => {
                    self.error("Expected ';' after statement");
                    Err(())
                }
            }
        } else {
            self.error("Expect identifier after 'let'");
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
                self.error("Expected ';' after expression");
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
                    self.error("Illegal assignment");
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
                self.parse_call()
            },
        }
    }
    fn parse_call(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.peek() == Token::LParen {
                self.advance();
                let mut arguments = vec![];
                if self.peek() != Token::RParen {
                    arguments.push(self.parse_expr()?);
                    while self.peek() == Token::Comma {
                        self.advance();
                        arguments.push(self.parse_expr()?);
                        if arguments.len() > 255 {
                            self.error("Cannot have more than 255 arguments");
                            return Err(())
                        }
                    }
                }
                if self.peek() != Token::RParen {
                    self.error("Expected ')' after arguments");
                    return Err(())
                }
                self.advance();
                expr = Expr::CallExpr(Box::from(expr), arguments);
            } else {
                break
            }
        }
        Ok(expr)
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
            Token::StringLiteral(literal) => {
                self.advance();
                Ok(Expr::LiteralExpr(Literal::StringLiteral(literal)))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                match self.peek() {
                    Token::RParen => {
                        self.advance();
                        Ok(expr)
                    },
                    _ => {
                        self.error("Expected ')' after expression");
                        Err(())
                    },
                }
            },
            _ => {
                self.advance();
                self.error("Unexpected Token");
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_1() {
        let tokens = vec![
            Token::If,
            Token::Ident("a".to_owned()),
            Token::Equal,
            Token::IntLiteral(10),
            Token::LBrace,
            Token::Return,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::Ident("a".to_owned()),
            Token::NotEqual,
            Token::IntLiteral(20),
            Token::LBrace,
            Token::Return,
            Token::Not,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::Ident("a".to_owned()),
            Token::Greater,
            Token::IntLiteral(20),
            Token::LBrace,
            Token::Return,
            Token::LParen,
            Token::Minus,
            Token::IntLiteral(30),
            Token::Plus,
            Token::IntLiteral(40),
            Token::RParen,
            Token::Multiply,
            Token::IntLiteral(50),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::Ident("a".to_owned()),
            Token::Less,
            Token::IntLiteral(30),
            Token::LBrace,
            Token::Return,
            Token::BooleanLiteral(true),
            Token::SemiColon,
            Token::RBrace,
            Token::Let,
            Token::Ident("x".to_owned()),
            Token::Assign,
            Token::StringLiteral("hello world!".to_owned()),
            Token::SemiColon,
            Token::Ident("print".to_owned()),
            Token::LParen,
            Token::Ident("x".to_owned()),
            Token::RParen,
            Token::SemiColon,
            Token::Return,
            Token::BooleanLiteral(false),
            Token::SemiColon,
            Token::EOF,
        ];
        let mut parser = Parser::new(&tokens);
        let result = parser.parse();
        for err in parser.errors.iter() {
            println!("{:?}", err);
        }
        assert_eq!(parser.errors.len(), 0);
        for stmt in result.iter() {
            println!("{:?}", stmt);
        }
    }
}