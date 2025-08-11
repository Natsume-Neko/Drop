use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::vm::Scope;

#[derive(Clone, Debug)]
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

    Print,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    String(String),
    Boolean(bool),
    Function(FunctionObject),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Int(value) => {
                write!(f, "{}", value);
            }
            Value::Boolean(value) => {
                write!(f, "{}", value);
            }
            Value::Function(_) => {
                write!(f, "FunctionObject");
            }
            Value::String(value) => {
                write!(f, "{}", value);
            }
            Value::None => {
                write!(f, "None");
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FunctionObject {
    pub params: Vec<String>,
    pub codes: Vec<Opcode>,
}

impl FunctionObject {
    pub fn new(params: Vec<String>, codes: Vec<Opcode>) -> Self {
        Self { params, codes }
    }
}
