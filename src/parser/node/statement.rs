use crate::lexer::token::{Token, Tokens};
use crate::parser::node::declare::DeclareNode;
use crate::parser::node::expression::ExpressionNode;

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Declare(DeclareStatementNode),
    Expression(ExpressionStatementNode),
    Return(ReturnStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek(0) {
            Some(token) => match token {
                Token::Return => StatementNode::Return(ReturnStatementNode::new(tokens)),
                Token::Type(_) => StatementNode::Declare(DeclareStatementNode::new(tokens)),
                _ => StatementNode::Expression(ExpressionStatementNode::new(tokens)),
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareStatementNode {
    pub declare: DeclareNode,
}
impl DeclareStatementNode {
    fn new(tokens: &mut Tokens) -> DeclareStatementNode {
        let msg = "ReturnStatementNode";
        let declare = DeclareNode::new(tokens);
        tokens.consume_semi().expect(msg);
        DeclareStatementNode { declare }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatementNode {
    pub expression: ExpressionNode,
}
impl ExpressionStatementNode {
    fn new(tokens: &mut Tokens) -> ExpressionStatementNode {
        let msg = "ExpressiontatementNode";
        let expression = ExpressionNode::new(tokens);
        tokens.consume_semi().expect(msg);
        ExpressionStatementNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatementNode {
    pub expression: ExpressionNode,
}
impl ReturnStatementNode {
    fn new(tokens: &mut Tokens) -> ReturnStatementNode {
        let msg = "ReturnStatementNode";
        tokens.consume_return().expect(msg);
        let expression = ExpressionNode::new(tokens);
        tokens.consume_semi().expect(msg);
        ReturnStatementNode { expression }
    }
}
