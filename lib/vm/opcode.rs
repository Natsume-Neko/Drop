#![allow(unused)]

#[derive(Clone)]
pub enum Opcode {
    Push(Value),
    Pop,
    Dup,

    Load(String),
    Store(String),
    Register(String),

    Call,
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
    Function(FunctionObject)
}

#[derive(Clone)]
pub struct FunctionObject {
    arity: usize,
    codes: Vec<Opcode>
}