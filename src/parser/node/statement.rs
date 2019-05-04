use crate::lexer::token::{Token, Tokens};
use crate::parser::node::declare::DeclareNode;
use crate::parser::node::expression::{ExpressionNode, ExpBaseNode);

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Declare(DeclareNode),
    Expression(ExpressionNode),
    Return(ReturnNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek(0) {
            Some(token) => match token {
                Token::Return => StatementNode::Return(ReturnNode::new(tokens)),
                Token::Type(_) => StatementNode::Declare(DeclareNode::new(tokens)),
                _ => StatementNode::Expression(ExpressionNode::new(tokens)), // TODO: impl error handling
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
    pub expression: ExpBaseNode,
}
impl ReturnNode {
    fn new(tokens: &mut Tokens) -> ReturnNode {
        tokens.consume_return().expect("return");
        let expression = ExpBaseNode::new(tokens);
        tokens.consume_semi().expect(";");
        ReturnNode { expression }
    }
}
