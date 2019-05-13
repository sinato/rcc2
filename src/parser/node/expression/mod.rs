pub mod binary;
pub mod unary;

use crate::emitter::emitter::Emitter;
use crate::emitter::environment::Value;
use crate::lexer::token::Tokens;
use crate::parser::node::expression::binary::BinaryNode;
use crate::parser::node::expression::unary::UnaryNode;

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    Unary(UnaryNode),
    Binary(BinaryNode),
}
impl ExpressionNode {
    pub fn new(tokens: &mut Tokens) -> ExpressionNode {
        BinaryNode::new(tokens)
    }
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        match self {
            ExpressionNode::Unary(node) => node.emit(emitter),
            ExpressionNode::Binary(node) => node.emit(emitter),
        }
    }
}
