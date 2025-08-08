#![allow(unused)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::vm::opcode::{FunctionObject, Opcode, Value};

pub mod opcode;

#[derive(Clone)]
pub struct Scope {
    variables: HashMap<String, Option<Value>>,
    upvalues: Option<Rc<RefCell<Scope>>>,
}

pub struct VM {
    code: Vec<Opcode>,
    stack: Vec<Value>,
    scope: Rc<RefCell<Scope>>,
    frames: Vec<CallFrame>,

    ip: usize,
    top: usize,
}

pub struct CallFrame {
    code: Vec<Opcode>,
    top: usize,
    ip: usize,
}

impl CallFrame {
    pub fn new(code: Vec<Opcode>, top: usize, ip: usize) -> Self {
        Self {
            code,
            top,
            ip,
        }
    }
}

impl Scope {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            upvalues: None,
        }
    }

    fn new_child(upvalues: Rc<RefCell<Scope>>) -> Self {
        Self {
            variables: HashMap::new(),
            upvalues: Some(upvalues)
        }
    }

}

impl VM {

    fn execute(&mut self, code: Opcode) -> usize {
        match code {
            Opcode::Push(value) => {
                self.stack.push(value);
                self.ip + 1
            }
            Opcode::Pop => {
                self.stack.pop();
                self.ip + 1
            }
            Opcode::Load(name) => {
                let mut scope = self.scope.clone();
                loop {
                    let scope_borrow = scope.borrow();
                    if scope_borrow.variables.contains_key(&name) {
                        let value = scope_borrow.variables.get(&name).unwrap();
                        if value.is_none() {
                            panic!("Can not use variable that is not given value: {}", name);
                        }
                        self.stack.push(value.as_ref().unwrap().clone());
                        break;
                    }
                    if scope_borrow.upvalues.is_none() {
                        panic!("No such variable: {}", name);
                    }
                    let parent = scope_borrow.upvalues.clone();
                    drop(scope_borrow);
                    scope = parent.unwrap();
                }
                self.ip + 1
            }
            Opcode::Store(name) => {
                let mut scope = self.scope.clone();
                if !scope.borrow().variables.contains_key(&name) {
                    panic!("No such variable: {}", name);
                }
                let value = self.stack.pop().unwrap();
                scope.borrow_mut().variables.insert(name, Some(value));
                self.ip + 1
            }
            Opcode::Register(name) => {
                self.scope.borrow_mut().variables.insert(name, None);
                self.ip + 1
            }
            Opcode::StoreFunction(name, params, codes) => {
                let scope = Rc::new(RefCell::new(Scope::new_child(self.scope.clone())));
                let func = FunctionObject::new(params, codes);
                self.scope.borrow_mut().variables.insert(name, Some(Value::Function(func)));
                self.ip + 1
            }
            Opcode::Call(num_args) => {
                let mut func = match self.stack.pop() {
                    Some(val) => match val {
                        Value::Function(func) => func,
                        _ => panic!("Can only call a function variable"),
                    }
                    None => unreachable!()
                };
                if num_args != func.params.len() {
                    panic!("The number of args is not true")
                }
                let new_scope = Rc::new(RefCell::new(Scope::new_child(self.scope.clone())));
                for param in func.params.iter().rev() {
                    let arg = match self.stack.pop() {
                        Some(val) => val,
                        None => unreachable!(),
                    };
                    new_scope.borrow_mut().variables.insert(param.clone(), Some(arg));
                }
                let mut old_code = func.codes;
                std::mem::swap(&mut self.code, &mut old_code);
                let callframe = CallFrame::new(old_code, self.top, self.ip);
                self.top = self.stack.len();
                self.scope = new_scope;
                self.frames.push(callframe);

                0
            }
            Opcode::Return => {
                let frame = match self.frames.pop() {
                    Some(frame) => frame,
                    None => panic!("Return should live in a function"),
                };
                if self.stack.len() > frame.top + 1 || self.stack.len() < frame.top {
                    panic!("Unknown Error: the call stack overflow or underflow (from a function return)")
                }
                if self.stack.len() == frame.top {
                    self.stack.push(Value::None);
                }
                self.top = frame.top;
                let old_scope = self.scope.clone();
                let new_scope = old_scope.borrow().upvalues.clone();
                let new_scope = match new_scope {
                    Some(parent) => parent,
                    _ => unreachable!()
                };
                self.scope = new_scope;
                std::mem::replace(&mut self.code, frame.code);
                frame.ip + 1
            }
            _ => unimplemented!(),
        }
    }
}