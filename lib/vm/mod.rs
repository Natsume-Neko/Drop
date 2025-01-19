use std::collections::HashMap;
use crate::vm::opcode::{Opcode, Value};

pub mod opcode;


#[derive(Debug, Clone)]
pub struct Scope {
    variables: HashMap<String, usize>,

}

pub struct VM {
    code: Vec<Opcode>,
    stack: Vec<Value>,

    ip: usize,
}