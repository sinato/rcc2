extern crate inkwell;

use std::{env, process};

mod emitter;
mod lexer;
mod parser;

use emitter::Emitter;
use lexer::Lexer;
use parser::parser;

fn compiler(code: String) {
    // let input = String::from("1 * 2");
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(code);
    // println!("{:?}", tokens);
    let node = parser(&mut tokens);
    // dbg!(node.clone());
    let emitter = Emitter::new();
    emitter.emit(node);
    emitter.print_to_file();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage rcc2 \"<code>\"");
        process::exit(1);
    }
    let code = args[1].to_string();
    compiler(code);
}
