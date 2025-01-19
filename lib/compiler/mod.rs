mod symbol;

use crate::parser::ast::{BinOp, Expr, Ident, Literal, Program, Stmt, UnaryOp};
use crate::vm::opcode::{Opcode, Value};

pub struct Compiler {
    codes: Vec<Opcode>
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            codes: vec![]
        }
    }
    pub fn compile(&mut self, program: &Program) {
        for stmt in program.iter() {
            self.compile_stmt(stmt)
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::ExprStmt(expr) => {
                self.compile_expr(expr)
            }
            Stmt::LetStmt(ident, expr) => {
                self.compile_let(ident, expr)
            }
            Stmt::FnStmt(ident, params, body) => {
                self.compile_fn(ident, params, body)
            }
            Stmt::BlockStmt(stmts) => {
                self.compile_block(stmts)
            }
            Stmt::IfStmt(condition, body, alt) => {
                self.compile_if(condition, body, alt)
            }
            Stmt::ReturnStmt(expr) => {
                self.compile_ret(expr)
            }
            Stmt::WhileStmt(condition, body) => {
                self.compile_while(condition, body)
            }
        }
    }

    fn compile_while(&mut self, condition: &Expr, body: &Box<Stmt>) {
        todo!()
    }

    fn compile_ret(&mut self, expr: &Option<Expr>) {
        todo!()
    }

    fn compile_if(&mut self, condition: &Expr, body: &Box<Stmt>, alt: &Option<Box<Stmt>>) {
        todo!()
    }

    fn compile_block(&mut self, stmts: &Vec<Stmt>) {
        todo!()
    }
    fn compile_fn(&mut self, ident: &Ident, params: &Vec<Expr>, body: &Box<Stmt>) {
        todo!()
    }

    fn compile_let(&mut self, ident: &Ident, expr: &Option<Expr>) {
        todo!()
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::LiteralExpr(literal) => self.compile_literal(literal),
            Expr::IdentExpr(ident) => self.compile_ident(ident),
            Expr::BinExpr(l_expr, op, r_expr) => {
                self.compile_binary(l_expr, op, r_expr)
            }
            Expr::UnaryExpr(op, expr) => {
                self.compile_unary(op, expr)
            }
            Expr::AssignmentExpr(ident, expr) => {
                self.compile_assignment(ident, expr)
            }
            Expr::CallExpr(func, args) => {

            }
        }
    }

    fn compile_call(&mut self, func: &Box<Expr>, args: &Vec<Expr>) {
        todo!()
    }
    fn compile_assignment(&mut self, ident: &Ident, expr: &Box<Expr>) {
        todo!()
    }
    fn compile_unary(&mut self, op: &UnaryOp, expr: &Box<Expr>) {
        todo!()
    }
    fn compile_binary(&mut self, l_expr: &Expr, op: &BinOp, r_expr: &Expr) {
        todo!()
    }
    fn compile_ident(&mut self, ident: &Ident) {
        todo!()
    }
    fn compile_literal(&mut self, literal: &Literal) {
        todo!()
    }
    fn emit(&mut self, code: Opcode) {
        self.codes.push(code)
    }
}