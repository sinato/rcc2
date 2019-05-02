extern crate inkwell;

use inkwell::context::Context;
use std::path;

mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::{BinaryExpNode, PrimaryNode};

fn binary_expression_parser(lhs_token: Token, rhs_token: Token, op_token: Token) -> BinaryExpNode {
    let lhs = PrimaryNode { token: lhs_token };
    let rhs = PrimaryNode { token: rhs_token };
    BinaryExpNode {
        op: op_token,
        lhs,
        rhs,
    }
}

fn binary_expression_emitter(node: BinaryExpNode) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // generate function
    let function = module.add_function("main", context.i64_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(&function, "entry");
    builder.position_at_end(&basic_block);

    // define main function
    let i64_type = context.i64_type();
    let const_x = i64_type.const_int(node.lhs.get_number(), false);
    let const_y = i64_type.const_int(node.rhs.get_number(), false);
    let sum = builder.build_int_add(const_x, const_y, "main");
    builder.build_return(Some(&sum));

    // print_to_file
    let _ = module.print_to_file(path::Path::new("compiled.ll"));
}

fn main() {
    let input = String::from("1 + 2");
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(input);
    println!("{:?}", tokens);
    let lhs = tokens.pop().unwrap();
    let op = tokens.pop().unwrap();
    let rhs = tokens.pop().unwrap();
    let node = binary_expression_parser(lhs, rhs, op);
    binary_expression_emitter(node);
}
