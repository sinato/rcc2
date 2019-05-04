pub mod declare;
pub mod expression;
pub mod statement;

use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::ExpBaseNode;
use crate::parser::node::statement::StatementNode;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub declares: Vec<TopLevelDeclareNode>,
}
impl Node {
    pub fn new(tokens: &mut Tokens) -> Node {
        let mut declares: Vec<TopLevelDeclareNode> = Vec::new();
        declares.push(TopLevelDeclareNode::new(tokens));
        Node { declares }
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    identifier: String,
    arguments: Vec<ExpBaseNode>, // TODO: change to Vec<declare>
    pub statements: Vec<StatementNode>,
}
impl FunctionNode {
    fn new(tokens: &mut Tokens) -> FunctionNode {
        let _function_type = tokens.consume_type().expect("type");
        let identifier = tokens.consume_identifier().expect("identifier");
        tokens.pop(); // consume ParenS
        let arguments = vec![];
        tokens.pop(); // consume ParenE
        tokens.pop(); // consume BlockS
        let mut statements: Vec<StatementNode> = Vec::new();
        loop {
            match tokens.peek(0) {
                Some(token) => match token {
                    Token::BlockE => break,
                    _ => {
                        let statement = StatementNode::new(tokens);
                        statements.push(statement);
                    }
                },
                None => panic!(),
            }
        }
        tokens.pop(); // consume BlockE
        FunctionNode {
            identifier,
            arguments,
            statements,
        }
    }
}
