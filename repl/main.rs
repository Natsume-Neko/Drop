extern crate drop_lib;
use std::io;
use std::io::Write;
use drop_lib::lexer;
const PROMPT: &str = ">> ";


fn main() {
    println!("Hello to the Drop Language!");
    println!("Feel free to type any commands!");
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = lexer::Lexer::lex_tokens(input);
                for tok in tokens.iter() {
                    println!("[{:?}]", tok);
                }
            }
            _ => {
                println!("IO Error!");
            }
        }
    }
}