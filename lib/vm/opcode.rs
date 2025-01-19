#![allow(unused)]

pub enum Opcode {
    Push(Value),
    Pop,
    Dup,

    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(usize),
    StoreGlobal(usize),

    Call(usize),
    Return,

    Jump(usize),
    JumpIfTrue(usize),
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

pub enum Value {
    Int(i64),
    String(String),
    Boolean(bool),
    Function(FunctionObject)
}

pub struct FunctionObject {
    entry_point: usize,
    arity: usize,
}