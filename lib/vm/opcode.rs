#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

use crate::vm::Scope;

#[derive(Clone)]
pub enum Opcode {
    Push(Value),
    Pop,

    Load(String),
    Store(String),
    Register(String),
    StoreFunction(String, Vec<String>, Vec<Opcode>),

    Call(usize),
    Return,

    Jump(usize),
    JumpIfFalse(usize),

    Add,
    Subtract,
    Multiply,
    Divide,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    Negate,
    Not,

    BeginScope,
    EndScope,

}

#[derive(Clone)]
pub enum Value {
    Int(i64),
    String(String),
    Boolean(bool),
    Function(FunctionObject),
    None,
}

#[derive(Clone)]
pub struct FunctionObject {
    pub params: Vec<String>,
    pub codes: Vec<Opcode>,
}

impl FunctionObject {
    pub fn new(params: Vec<String>, codes: Vec<Opcode>) -> Self {
        Self {
            params,
            codes,
        }
    }
}