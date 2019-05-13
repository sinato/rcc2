pub mod prefix;
pub mod primary;
pub mod suffix;

use crate::emitter::emitter::Emitter;
use crate::emitter::environment::Value;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::unary::prefix::PrefixNode;
use crate::parser::node::expression::unary::primary::PrimaryNode;
use crate::parser::node::expression::unary::suffix::{
    ArrayAccessNode, FunctionCallNode, SuffixNode,
};

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryNode {
    Primary(PrimaryNode),
    Prefix(PrefixNode),
    Suffix(SuffixNode),
}
impl UnaryNode {
    pub fn new(tokens: &mut Tokens) -> UnaryNode {
        match tokens.peek(0) {
            Some(token) => match token {
                Token::Op(op, _) => match op.as_ref() {
                    "*" | "&" => UnaryNode::new_with_prefix(tokens),
                    _ => UnaryNode::new_with_suffix(tokens),
                },
                _ => UnaryNode::new_with_suffix(tokens),
            },
            None => UnaryNode::new_with_suffix(tokens),
        }
    }
    fn new_with_prefix(tokens: &mut Tokens) -> UnaryNode {
        let (op, _property) = tokens
            .consume_operator()
            .expect("UnaryNode, new_with_prefix");
        match op.as_ref() {
            "*" | "&" => UnaryNode::Prefix(PrefixNode {
                op,
                val: PrimaryNode::new(tokens),
            }),
            _ => panic!(),
        }
    }
    fn new_with_suffix(tokens: &mut Tokens) -> UnaryNode {
        match tokens.peek(1) {
            Some(token) => match token {
                Token::SquareS => {
                    UnaryNode::Suffix(SuffixNode::Array(ArrayAccessNode::new(tokens)))
                }
                Token::ParenS => {
                    UnaryNode::Suffix(SuffixNode::FunctionCall(FunctionCallNode::new(tokens)))
                }
                _ => UnaryNode::Primary(PrimaryNode::new(tokens)),
            },
            None => UnaryNode::Primary(PrimaryNode::new(tokens)),
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        match self {
            UnaryNode::Primary(node) => node.emit(emitter),
            UnaryNode::Prefix(node) => node.emit(emitter),
            UnaryNode::Suffix(node) => node.emit(emitter),
        }
    }
}
