pub mod declare;
pub mod expression;
pub mod function;
pub mod statement;

use crate::emitter::emitter::Emitter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::function::FunctionNode;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub declares: Vec<TopLevelDeclareNode>,
}
impl Node {
    pub fn new(tokens: &mut Tokens) -> Node {
        // TODO: support this case -> `func() {}` (not `int func() {}`)
        let mut declares: Vec<TopLevelDeclareNode> = Vec::new();
        while let Some(Token::Type(_)) = tokens.peek(0) {
            declares.push(TopLevelDeclareNode::new(tokens));
        }
        Node { declares }
    }
    pub fn emit(self, emitter: &mut Emitter) {
        let mut declares = self.declares;
        declares.reverse();
        while let Some(declare) = declares.pop() {
            declare.emit(emitter)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TopLevelDeclareNode {
    Function(FunctionNode),
}
impl TopLevelDeclareNode {
    fn new(tokens: &mut Tokens) -> TopLevelDeclareNode {
        TopLevelDeclareNode::Function(FunctionNode::new(tokens))
    }
    pub fn emit(self, emitter: &mut Emitter) {
        match self {
            TopLevelDeclareNode::Function(node) => node.emit(emitter),
        }
    }
}
