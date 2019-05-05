pub mod direct;
pub mod pointer;

use inkwell::values::IntValue;

use crate::emitter::emitter::Emitter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::declare::direct::DirectDeclareNode;
use crate::parser::node::declare::pointer::PointerDeclareNode;

#[derive(Debug, PartialEq, Clone)]
pub enum DeclareNode {
    Direct(DirectDeclareNode),
    Pointer(PointerDeclareNode),
}
impl DeclareNode {
    pub fn new(tokens: &mut Tokens) -> DeclareNode {
        // expect Token::Type(_) as tokens.peek(0)
        match tokens.peek(1) {
            Some(token) => match token {
                Token::Ide(_identifier) => DeclareNode::Direct(DirectDeclareNode::new(tokens)),
                Token::Op(op, _) => match op.as_ref() {
                    "*" => DeclareNode::Pointer(PointerDeclareNode::new(tokens)),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => panic!(),
        }
    }
    pub fn get_identifier(&self) -> String {
        match self.clone() {
            DeclareNode::Direct(node) => match node {
                DirectDeclareNode::Variable(node) => node.identifier,
                DirectDeclareNode::Array(node) => node.identifier,
            },
            DeclareNode::Pointer(node) => node.identifier,
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let const_zero = emitter.context.i32_type().const_int(0, false);
        match self {
            DeclareNode::Direct(node) => match node {
                DirectDeclareNode::Variable(node) => match node.init_expression {
                    Some(expression) => {
                        let identifier = node.identifier;
                        let alloca = emitter
                            .builder
                            .build_alloca(emitter.context.i32_type(), &identifier);
                        emitter.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        expression.emit(emitter)
                    }
                    None => {
                        let identifier = node.identifier;
                        let alloca = emitter
                            .builder
                            .build_alloca(emitter.context.i32_type(), &identifier);
                        emitter.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        const_zero
                    }
                },
                DirectDeclareNode::Array(node) => {
                    let identifier = node.identifier;
                    let array_type = emitter.context.i32_type().array_type(node.init_size);
                    let alloca = match emitter.environment.get(&identifier) {
                        Some(_) => panic!(format!("redefinition of {}", identifier)),
                        None => emitter.builder.build_alloca(array_type, &identifier),
                    };
                    emitter.environment.update(identifier, alloca);
                    const_zero
                }
            },
            DeclareNode::Pointer(node) => {
                let identifier = node.identifier;
                let alloca = emitter
                    .builder
                    .build_alloca(emitter.context.i32_type(), &identifier);
                emitter.environment.update(identifier, alloca); // TODO: impl detect redefinition
                emitter.context.i32_type().const_int(0, false)
            }
        }
    }
}
