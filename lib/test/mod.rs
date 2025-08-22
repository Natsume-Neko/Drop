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
                let c = \"baka\";
                let d = \"hentai\";
                if a < b {
                    print(c);
                } else {
                    print(d);
                }
                let x = \"hello world!\";\
                print(x);\
            }\
            main();\
            ",
    );
    let lex_result = Lexer::lex_tokens(s.as_str());
    let mut parser = Parser::new(&lex_result);
    let parse_result = parser.parse();
    // println!("{:?}", parse_result);
    for err in parser.errors.iter() {
        println!("{:?}", err);
    }
    assert_eq!(parser.errors.len(), 0);
    let mut compiler = Compiler::new();
    compiler.compile(&parse_result);
    let mut vm = VM::new(compiler.codes);
    assert_eq!(vm.run(), Ok(()));
}

#[test]
fn run_all_2() {
    let s = String::from(
        "\
            fn fib(a, b, n) {\
                if n <= 0 {\
                    return b;\
                }\
                return fib(b, a + b, n - 1);\
            }\
            fn main() {\
                print(fib(1, 1, 20));\
            }\
            main();\
            ",
    );
    let lex_result = Lexer::lex_tokens(s.as_str());
    let mut parser = Parser::new(&lex_result);
    let parse_result = parser.parse();
    // println!("{:?}", parse_result);
    for err in parser.errors.iter() {
        println!("{:?}", err);
    }
    assert_eq!(parser.errors.len(), 0);
    let mut compiler = Compiler::new();
    compiler.compile(&parse_result);
    let mut vm = VM::new(compiler.codes);
    assert_eq!(vm.run(), Ok(()));
}

#[test]
fn run_all_3() {
    let s = String::from(
        "\
            fn fib(n) {\
                if n <= 1 {\
                    return 1;\
                }\
                return fib(n - 1) + fib(n - 2);\
            }\
            fn main() {\
                print(fib(21));\
            }\
            main();\
            ",
    );
    let lex_result = Lexer::lex_tokens(s.as_str());
    let mut parser = Parser::new(&lex_result);
    let parse_result = parser.parse();
    // println!("{:?}", parse_result);
    for err in parser.errors.iter() {
        println!("{:?}", err);
    }
    assert_eq!(parser.errors.len(), 0);
    let mut compiler = Compiler::new();
    compiler.compile(&parse_result);
    let mut vm = VM::new(compiler.codes);
    assert_eq!(vm.run(), Ok(()));
}

#[test]
fn run_all_4() {
    let s = String::from(
        "\
            fn test(n) {\
                if n <= 1 {\
                    return;\
                }\
                test(n - 1);\
                print(n);\
            }\
            fn main() {\
                test(3);\
            }\
            main();\
            ",
    );
    let lex_result = Lexer::lex_tokens(s.as_str());
    let mut parser = Parser::new(&lex_result);
    let parse_result = parser.parse();
    // println!("{:?}", parse_result);
    for err in parser.errors.iter() {
        println!("{:?}", err);
    }
    assert_eq!(parser.errors.len(), 0);
    let mut compiler = Compiler::new();
    compiler.compile(&parse_result);
    // println!("{:?}", compiler.codes);
    let mut vm = VM::new(compiler.codes);
    assert_eq!(vm.run(), Ok(()));
}

#[test]
fn run_all_5() {
    let s = String::from(
        "\
            fn main() {\
                print(\"hello world\");\
            }\
            main();\
            ",
    );
    let lex_result = Lexer::lex_tokens(s.as_str());
    let mut parser = Parser::new(&lex_result);
    let parse_result = parser.parse();
    // println!("{:?}", parse_result);
    for err in parser.errors.iter() {
        println!("{:?}", err);
    }
    assert_eq!(parser.errors.len(), 0);
    let mut compiler = Compiler::new();
    compiler.compile(&parse_result);
    let mut vm = VM::new(compiler.codes);
    assert_eq!(vm.run(), Ok(()));
}

#[test]
fn run_all_6() {
    let s = String::from(
        "\
            fn test2(n) {\
                if n <= 1 {\
                    return 1;\
                }\
                return 2;\
            }\
            fn test(n) {\
                if n <= 1 {\
                    return 1;\
                }\
                test2(n - 1);\
                print(n);\
            }\
            fn main() {\
                test(4);\
            }\
            main();\
            ",
    );
    let lex_result = Lexer::lex_tokens(s.as_str());
    let mut parser = Parser::new(&lex_result);
    let parse_result = parser.parse();
    // println!("{:?}", parse_result);
    for err in parser.errors.iter() {
        println!("{:?}", err);
    }
    assert_eq!(parser.errors.len(), 0);
    let mut compiler = Compiler::new();
    compiler.compile(&parse_result);
    let mut vm = VM::new(compiler.codes);
    assert_eq!(vm.run(), Ok(()));
}