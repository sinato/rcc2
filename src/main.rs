extern crate inkwell;
extern crate rcc2;

use std::{env, process};

use rcc2::emitter::emitter::Emitter;
use rcc2::lexer::Lexer;
use rcc2::parser::parser;

fn compiler(code: String) {
    // let input = String::from("1 * 2");
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(code);
    // dbg!(tokens.clone());
    let node = parser(&mut tokens);
    // dbg!(node.clone());
    let mut emitter = Emitter::new();
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
