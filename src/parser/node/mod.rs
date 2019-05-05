pub mod declare;
pub mod expression;
pub mod statement;

use crate::lexer::token::{Token, Tokens};
use crate::parser::node::declare::DeclareNode;
use crate::parser::node::statement::StatementNode;

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
    pub identifier: String,
    pub arguments: Vec<DeclareNode>,
    pub statements: Vec<StatementNode>,
}
impl FunctionNode {
    fn new(tokens: &mut Tokens) -> FunctionNode {
        let msg = "FunctionNode";
        let _function_type = tokens.consume_type().expect("type");
        let identifier = tokens.consume_identifier().expect("identifier");
        tokens.consume_paren_s().expect(msg);
        let mut arguments = vec![];
        while let Some(Token::Type(_)) = tokens.peek(0) {
            let argument = DeclareNode::new(tokens);
            arguments.push(argument);
            if let Some(Token::Comma) = tokens.peek(0) {
                tokens.pop();
            }
        }
        tokens.consume_paren_e().expect(msg);
        tokens.consume_block_s().expect(msg);
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
        tokens.consume_block_e().expect(msg);
        FunctionNode {
            identifier,
            arguments,
            statements,
        }
    }
}
