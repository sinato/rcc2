extern crate inkwell;

use inkwell::context::Context;
use std::{env, path, process};

mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::{parser, Node};

fn binary_expression_emitter(node: Node) {
    match node {
        Node::BinaryExp(node) => {
            // initialize
            let context = Context::create();
            let module = context.create_module("my_module");
            let builder = context.create_builder();

            // generate function
            let function =
                module.add_function("main", context.i64_type().fn_type(&[], false), None);
            let basic_block = context.append_basic_block(&function, "entry");
            builder.position_at_end(&basic_block);

            // define main function
            let i64_type = context.i64_type();
            let lhs = *node.lhs;
            let rhs = *node.rhs;
            let const_x = i64_type.const_int(lhs.get_number(), false);
            let const_y = i64_type.const_int(rhs.get_number(), false);
            let ret = match node.op {
                Token::Op(op, _) => match op.as_ref() {
                    "+" => builder.build_int_add(const_x, const_y, "main"),
                    "*" => builder.build_int_mul(const_x, const_y, "main"),
                    _ => panic!("Operator not implemented."),
                },
                _ => panic!(),
            };
            builder.build_return(Some(&ret));

            // print_to_file
            let _ = module.print_to_file(path::Path::new("compiled.ll"));
        }
        _ => panic!(),
    }
}

fn compiler(code: String) {
    // let input = String::from("1 * 2");
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(code);
    println!("{:?}", tokens);
    let node = parser(&mut tokens);
    binary_expression_emitter(node);
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
