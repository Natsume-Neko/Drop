#[cfg(test)]
use crate::{compiler::Compiler, lexer::Lexer, parser::Parser, vm::VM};

#[test]
fn run_all_1() {
    let s = String::from(
        "\
            fn add(a, b) {\
                return a + b;\
            }\
            fn main() {\
                let a = 100;
                let b = true;
                print(add(a, b));
                let x = \"hello world!\";\
                print(x);\
            }\
            main();\
            ",
    );
    let lex_result = Lexer::lex_tokens(s.as_str());
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
