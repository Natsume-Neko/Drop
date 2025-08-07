#![allow(unused)]

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
                self.compile_expr(expr);
                self.emit(Opcode::Pop)
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
        let start_pos = self.codes.len();
        self.compile_expr(condition);
        self.emit(Opcode::JumpIfFalse(0));
        let backpatch = self.codes.len() - 1;
        let body = match body.as_ref() {
            Stmt::BlockStmt(block) => block,
            _ => unreachable!(),
        };
        self.compile_block(body);
        self.emit(Opcode::Jump(start_pos));
        let pos = self.codes.len();
        self.codes[backpatch] = Opcode::JumpIfFalse(pos);
    }

    fn compile_ret(&mut self, expr: &Option<Expr>) {
        if let Some(expression) = expr {
            self.compile_expr(expression);
        }
        self.emit(Opcode::Return);
    }

    fn compile_if(&mut self, condition: &Expr, body: &Box<Stmt>, alt: &Option<Box<Stmt>>) {
        self.compile_expr(condition);
        self.emit(Opcode::JumpIfFalse(0));
        let backpatch1 = self.codes.len() - 1;
        let body = match body.as_ref() {
            Stmt::BlockStmt(block) => block,
            _ => unreachable!()
        };
        let pos1 = self.codes.len();
        self.codes[backpatch1] = Opcode::JumpIfFalse(pos1);
        match alt {
            Some(content) => {
                self.emit(Opcode::Jump(0));
                let backpatch2 = self.codes.len() - 1;
                let alt_body = match content.as_ref() {
                    Stmt::BlockStmt(block) => block,
                    _ => unreachable!()
                };
                self.compile_block(alt_body);
                let pos2 = self.codes.len();
                self.codes[backpatch2] = Opcode::Jump(pos2);
            }
            None => (),
        };
    }

    fn compile_block(&mut self, stmts: &Vec<Stmt>) {
        self.emit(Opcode::BeginScope);
        let mut sub_compiler = Compiler::new();
        sub_compiler.compile(stmts);
        self.codes.append(&mut sub_compiler.codes);
        self.emit(Opcode::EndScope);
    }

    fn compile_fn(&mut self, ident: &Ident, params: &Vec<Expr>, body: &Box<Stmt>) {
        let mut param_names = vec![];
        for param in params {
            match param {
                Expr::IdentExpr(name) => {
                    param_names.push(name.0.to_string());
                }
                _ => unreachable!()
            }
        }
        let body = match body.as_ref() {
            Stmt::BlockStmt(block) => block,
            _ => unreachable!()
        };
        let mut sub_compiler = Compiler::new();
        sub_compiler.compile(body);
        self.emit(Opcode::StoreFunction(ident.0.to_string(), param_names, sub_compiler.codes));
    }

    fn compile_let(&mut self, ident: &Ident, expr: &Option<Expr>) {
        if let Some(expression) = expr {
            self.compile_expr(expression);
            self.emit(Opcode::Register(ident.0.to_string()));
            self.emit(Opcode::Store(ident.0.to_string()));
            self.emit(Opcode::Pop);
        } else {
            self.emit(Opcode::Register(ident.0.to_string()));
        }
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
                self.compile_call(func, args)
            }
        }
    }

    fn compile_call(&mut self, func: &Box<Expr>, args: &Vec<Expr>) {
        for arg in args {
            self.compile_expr(arg);
        }
        match func.as_ref() {
            Expr::CallExpr(expr, new_args) => {
                self.compile_call(expr, new_args);
            }
            Expr::IdentExpr(ident) => {
                self.compile_ident(ident);
            }
            _ => {
                panic!("The function call should lead by ident")
            }
        }
        self.emit(Opcode::Call);
    }
    fn compile_assignment(&mut self, ident: &Ident, expr: &Box<Expr>) {
        self.compile_expr(expr);
        self.emit(Opcode::Store(ident.0.to_string()));
    }
    fn compile_unary(&mut self, op: &UnaryOp, expr: &Box<Expr>) {
        self.compile_expr(expr);
        match op {
            UnaryOp::Not => self.emit(Opcode::Not),
            UnaryOp::UnaryMinus => self.emit(Opcode::Negate),
            UnaryOp::UnaryPlus => (),
        }
    }
    fn compile_binary(&mut self, l_expr: &Expr, op: &BinOp, r_expr: &Expr) {
        self.compile_expr(l_expr);
        self.compile_expr(r_expr);
        match op {
            BinOp::Plus => self.emit(Opcode::Add),
            BinOp::Minus => self.emit(Opcode::Subtract),
            BinOp::Multiply => self.emit(Opcode::Multiply),
            BinOp::Divide => self.emit(Opcode::Divide),
            BinOp::Equal => self.emit(Opcode::Equal),
            BinOp::Greater => self.emit(Opcode::Greater),
            BinOp::GreaterEqual => self.emit(Opcode::GreaterEqual),
            BinOp::Less => self.emit(Opcode::Less),
            BinOp::LessEqual => self.emit(Opcode::LessEqual),
            BinOp::NotEqual => self.emit(Opcode::NotEqual),
        }
    }
    fn compile_ident(&mut self, ident: &Ident) {
        self.emit(Opcode::Load(ident.0.to_string()));
    }
    fn compile_literal(&mut self, literal: &Literal) {
        match literal {
            Literal::BoolLiteral(val) => {
                self.emit(Opcode::Push(Value::Boolean(*val)));
            }
            Literal::IntLiteral(val) => {
                self.emit(Opcode::Push(Value::Int(*val)));
            }
            Literal::StringLiteral(val) => {
                self.emit(Opcode::Push(Value::String(val.to_string())));
            }
        }
    }
    fn emit(&mut self, code: Opcode) {
        self.codes.push(code)
    }
}