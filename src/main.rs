use std::{env, fs};

use drop_lib::{compiler::Compiler, lexer::Lexer, parser::Parser, vm::VM};

extern crate drop_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("You should pass exactly 2 args")
    }
    let file_name = args.last().unwrap().clone();
    let file_content = match fs::read_to_string(&file_name) {
        Ok(content) => content,
        Err(e) => {
            panic!("{:?}", e);
        } 
    };

    let lex_result = Lexer::lex_tokens(file_content.as_str());
    let mut parser = Parser::new(&lex_result);
    let parse_result = parser.parse();
    for err in parser.errors.iter() {
        println!("{:?}", err);
    }
    assert_eq!(parser.errors.len(), 0);
    let mut compiler = Compiler::new();
    compiler.compile(&parse_result);
    let mut vm = VM::new(compiler.codes);
    assert_eq!(vm.run(), Ok(()));
}
