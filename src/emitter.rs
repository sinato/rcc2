use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;

use std::path;

use crate::lexer::Token;
use crate::parser::{BinaryNode, ExpNode, Node, PrimaryNode};

pub struct Emitter {
    context: Context,
    builder: Builder,
    module: Module,
}
impl Emitter {
    pub fn new() -> Emitter {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("my_module");
        Emitter {
            context,
            builder,
            module,
        }
    }
    pub fn print_to_file(&self) {
        let _ = self.module.print_to_file(path::Path::new("compiled.ll"));
    }
    pub fn emit(&self, node: Node) {
        // generate function
        let function =
            self.module
                .add_function("main", self.context.i32_type().fn_type(&[], false), None);
        let basic_block = self.context.append_basic_block(&function, "entry");
        self.builder.position_at_end(&basic_block);

        match node {
            Node::Exp(node) => {
                let ret = self.emit_expression(node);
                self.builder.build_return(Some(&ret));
            }
        }
    }
    pub fn emit_expression(&self, node: ExpNode) -> IntValue {
        match node {
            ExpNode::Binary(node) => self.emit_binary(node),
            ExpNode::Primary(node) => self.emit_primary(node),
        }
    }
    fn emit_binary(&self, node: BinaryNode) -> IntValue {
        // define main function
        let const_lhs = self.emit_expression(*node.lhs);
        let const_rhs = self.emit_expression(*node.rhs);
        let ret = match node.op {
            Token::Op(op, _) => match op.as_ref() {
                "+" => self.builder.build_int_add(const_lhs, const_rhs, "main"),
                "*" => self.builder.build_int_mul(const_lhs, const_rhs, "main"),
                _ => panic!("Operator not implemented."),
            },
            _ => panic!(),
        };
        ret
    }
    fn emit_primary(&self, node: PrimaryNode) -> IntValue {
        let num = node.get_number_u64();
        self.context.i32_type().const_int(num, false)
    }
}