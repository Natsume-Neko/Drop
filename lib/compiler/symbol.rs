use std::collections::HashMap;

pub struct Symbol(usize);
pub struct Scope(HashMap<String, Symbol>);
pub struct SymbolTable(Vec<Scope>);