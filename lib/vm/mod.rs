#![allow(unused)]
use std::collections::HashMap;
use crate::vm::opcode::{Opcode, Value};

pub mod opcode;


#[derive(Clone)]
pub struct Scope<'a> {
    variables: HashMap<String, Value>,
    upvalues: Option<&'a Scope<'a>>,
}

pub struct VM {
    code: Vec<Opcode>,
    stack: Vec<Value>,

    ip: usize,
}