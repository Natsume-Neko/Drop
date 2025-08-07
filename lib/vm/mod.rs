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

    ip: usize,
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
                        } else {
                            self.stack.push(value.as_ref().unwrap().clone());
                            break;
                        }
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
                let func = FunctionObject::new(params, codes, scope);
                self.scope.borrow_mut().variables.insert(name, Some(Value::Function(func)));
                self.ip + 1
            }
            _ => unimplemented!(),
        }
    }
}