pub mod direct;
pub mod pointer;

use crate::emitter::emitter::Emitter;
use crate::emitter::environment::Value;
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
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        match self {
            DeclareNode::Direct(node) => node.emit(emitter),
            DeclareNode::Pointer(node) => node.emit(emitter),
        }
    }
}
