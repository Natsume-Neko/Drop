use crate::vm::opcode::{FunctionObject, Opcode, Scope, Value};
use std::{cell::RefCell, rc::Rc};

pub mod opcode;



pub struct VM {
    code: Vec<Opcode>,
    stack: Vec<Value>,
    scope: Rc<RefCell<Scope>>,
    frames: Vec<CallFrame>,

    ip: usize,
}

pub struct CallFrame {
    code: Vec<Opcode>,
    scope: Rc<RefCell<Scope>>,
    top: usize,
    ip: usize,
}

impl CallFrame {
    pub fn new(code: Vec<Opcode>, scope: Rc<RefCell<Scope>>,top: usize, ip: usize) -> Self {
        Self { code, scope, top, ip }
    }
}



impl VM {
    pub fn new(code: Vec<Opcode>) -> Self {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let print_func = FunctionObject::new(
            vec!["value".to_string()],
            vec![
                Opcode::Load("value".to_string()),
                Opcode::Print,
                Opcode::Return,
            ],
            Rc::new(RefCell::new(Scope::new())),
        );
        scope
            .borrow_mut()
            .variables
            .insert("print".to_string(), Some(Value::Function(print_func)));
        Self {
            code,
            stack: vec![],
            scope,
            frames: vec![],
            ip: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.ip == self.code.len() {
                break;
            }
            if self.ip > self.code.len() {
                return Err("Unknown Error: ip exceed the code length".to_string());
            }
            self.ip = self.execute(self.code.get(self.ip).unwrap().clone());
        }
        Ok(())
    }

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
                loop {
                    let mut scope_borrow = scope.borrow_mut();
                    if scope_borrow.variables.contains_key(&name) {
                        let value = self.stack.last().unwrap().clone();
                        scope_borrow.variables.insert(name, Some(value));
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
            Opcode::Register(name) => {
                self.scope.borrow_mut().variables.insert(name, None);
                self.ip + 1
            }
            Opcode::StoreFunction(name, params, codes) => {
                let func = FunctionObject::new(params, codes, self.scope.clone());
                self.scope
                    .borrow_mut()
                    .variables
                    .insert(name, Some(Value::Function(func)));
                self.ip + 1
            }
            Opcode::Call(num_args) => {
                let func = match self.stack.pop() {
                    Some(val) => match val {
                        Value::Function(func) => func,
                        _ => panic!("Can only call a function variable"),
                    },
                    None => unreachable!(),
                };
                if num_args != func.params.len() {
                    panic!("The number of args is not true")
                }
                let new_scope = Rc::new(RefCell::new(Scope::new_child(func.up_scope.clone())));
                for param in func.params.iter().rev() {
                    let arg = match self.stack.pop() {
                        Some(val) => val,
                        None => unreachable!(),
                    };
                    new_scope
                        .borrow_mut()
                        .variables
                        .insert(param.clone(), Some(arg));
                }
                let mut old_code = func.codes;
                std::mem::swap(&mut self.code, &mut old_code);
                let callframe = CallFrame::new(old_code, self.scope.clone(), self.stack.len(), self.ip);
                self.scope = new_scope;
                self.frames.push(callframe);

                0
            }
            Opcode::Return => {
                let frame = match self.frames.pop() {
                    Some(frame) => frame,
                    None => panic!("Return should live in a function"),
                };
                if self.stack.len() > frame.top + 1 {
                    panic!("Unknown Error: the call stack overflow (from a function return)")
                }
                if self.stack.len() < frame.top {
                    panic!("Unknown Error: the call stack underflow (from a function return)")
                }
                if self.stack.len() == frame.top {
                    self.stack.push(Value::None);
                }
                // println!("{:?}", self.stack);
                // println!("{:?}", self.scope);
                self.scope = frame.scope.clone();
                let _ = std::mem::replace(&mut self.code, frame.code);
                frame.ip + 1
            }
            Opcode::BeginScope => {
                let new_scope = Rc::new(RefCell::new(Scope::new_child(self.scope.clone())));
                self.scope = new_scope;
                self.ip + 1
            }
            Opcode::EndScope => {
                let old_scope = self.scope.clone();
                let new_scope = old_scope.borrow().upvalues.clone();
                let new_scope = match new_scope {
                    Some(parent) => parent,
                    _ => panic!("Cannot end the root scope"),
                };
                self.scope = new_scope;
                self.ip + 1
            }
            Opcode::Jump(pos) => pos,
            Opcode::JumpIfFalse(pos) => match self.stack.pop() {
                Some(value) => match value {
                    Value::Boolean(val) => match val {
                        true => self.ip + 1,
                        false => pos,
                    },
                    Value::Int(val) => match val {
                        0 => pos,
                        _ => self.ip + 1,
                    },
                    _ => {
                        panic!("Expression in if condition should be boolean or int")
                    }
                },
                _ => panic!("Unknown Error: stack empty"),
            },
            Opcode::Add => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let sum = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int((value1 as i64) + (value2 as i64)),
                        Value::Int(value2) => Value::Int((value1 as i64) + value2),
                        _ => panic!("You cannot add these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int(value1 + (value2 as i64)),
                        Value::Int(value2) => Value::Int(value1 + value2),
                        _ => panic!("You cannot add these different value type"),
                    },
                    Value::String(mut value1) => match value2 {
                        Value::String(value2) => {
                            value1.push_str(value2.as_str());
                            Value::String(value1)
                        }
                        _ => panic!("You cannot add these different value type"),
                    },
                    _ => panic!("You cannot add these different value type"),
                };
                self.stack.push(sum);
                self.ip + 1
            }
            Opcode::Subtract => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let diff = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int((value1 as i64) - (value2 as i64)),
                        Value::Int(value2) => Value::Int((value1 as i64) - value2),
                        _ => panic!("You cannot substract these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int(value1 - (value2 as i64)),
                        Value::Int(value2) => Value::Int(value1 - value2),
                        _ => panic!("You cannot substract these different value type"),
                    },
                    _ => panic!("You cannot substract these different value type"),
                };
                self.stack.push(diff);
                self.ip + 1
            }
            Opcode::Multiply => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let mul = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int((value1 as i64) * (value2 as i64)),
                        Value::Int(value2) => Value::Int((value1 as i64) * value2),
                        _ => panic!("You cannot multiply these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int(value1 * (value2 as i64)),
                        Value::Int(value2) => Value::Int(value1 * value2),
                        _ => panic!("You cannot multiply these different value type"),
                    },
                    _ => panic!("You cannot multiply these different value type"),
                };
                self.stack.push(mul);
                self.ip + 1
            }
            Opcode::Divide => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let div = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int((value1 as i64) / (value2 as i64)),
                        Value::Int(value2) => Value::Int((value1 as i64) / value2),
                        _ => panic!("You cannot divide these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Int(value1 / (value2 as i64)),
                        Value::Int(value2) => Value::Int(value1 / value2),
                        _ => panic!("You cannot divide these different value type"),
                    },
                    _ => panic!("You cannot divide these different value type"),
                };
                self.stack.push(div);
                self.ip + 1
            }
            Opcode::Less => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean((value1 as i64) < (value2 as i64)),
                        Value::Int(value2) => Value::Boolean((value1 as i64) < value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean(value1 < (value2 as i64)),
                        Value::Int(value2) => Value::Boolean(value1 < value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::String(value1) => match value2 {
                        Value::String(value2) => Value::Boolean(value1.lt(&value2)),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    _ => panic!("You cannot compare these different value type"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::Greater => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean((value1 as i64) > (value2 as i64)),
                        Value::Int(value2) => Value::Boolean((value1 as i64) > value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean(value1 > (value2 as i64)),
                        Value::Int(value2) => Value::Boolean(value1 > value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::String(value1) => match value2 {
                        Value::String(value2) => Value::Boolean(value1.gt(&value2)),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    _ => panic!("You cannot compare these different value type"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::LessEqual => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => {
                            Value::Boolean((value1 as i64) <= (value2 as i64))
                        }
                        Value::Int(value2) => Value::Boolean((value1 as i64) <= value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean(value1 <= (value2 as i64)),
                        Value::Int(value2) => Value::Boolean(value1 <= value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::String(value1) => match value2 {
                        Value::String(value2) => Value::Boolean(value1.le(&value2)),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    _ => panic!("You cannot compare these different value type"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::GreaterEqual => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => {
                            Value::Boolean((value1 as i64) >= (value2 as i64))
                        }
                        Value::Int(value2) => Value::Boolean((value1 as i64) >= value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean(value1 >= (value2 as i64)),
                        Value::Int(value2) => Value::Boolean(value1 >= value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::String(value1) => match value2 {
                        Value::String(value2) => Value::Boolean(value1.ge(&value2)),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    _ => panic!("You cannot compare these different value type"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::Equal => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => {
                            Value::Boolean((value1 as i64) == (value2 as i64))
                        }
                        Value::Int(value2) => Value::Boolean((value1 as i64) == value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean(value1 == (value2 as i64)),
                        Value::Int(value2) => Value::Boolean(value1 == value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::String(value1) => match value2 {
                        Value::String(value2) => Value::Boolean(value1.eq(&value2)),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    _ => panic!("You cannot compare these different value type"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::NotEqual => {
                let value2 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let value1 = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value1 {
                    Value::Boolean(value1) => match value2 {
                        Value::Boolean(value2) => {
                            Value::Boolean((value1 as i64) != (value2 as i64))
                        }
                        Value::Int(value2) => Value::Boolean((value1 as i64) != value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::Int(value1) => match value2 {
                        Value::Boolean(value2) => Value::Boolean(value1 != (value2 as i64)),
                        Value::Int(value2) => Value::Boolean(value1 != value2),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    Value::String(value1) => match value2 {
                        Value::String(value2) => Value::Boolean(value1.ne(&value2)),
                        _ => panic!("You cannot compare these different value type"),
                    },
                    _ => panic!("You cannot compare these different value type"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::Negate => {
                let value = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value {
                    Value::Boolean(value) => Value::Int(-(value as i64)),
                    Value::Int(value) => Value::Int(-value),
                    _ => panic!("You can only negate a number"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::Not => {
                let value = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                let result = match value {
                    Value::Boolean(value) => Value::Boolean(!value),
                    Value::Int(value) => match value {
                        0 => Value::Boolean(true),
                        _ => Value::Boolean(false),
                    },
                    _ => panic!("You can only Not a Boolean or Number"),
                };
                self.stack.push(result);
                self.ip + 1
            }
            Opcode::Print => {
                let value = match self.stack.pop() {
                    Some(value) => value,
                    _ => panic!("Unknown Error: stack empty"),
                };
                println!("{}", value);
                self.ip + 1
            }
        }
    }
}
