pub mod expression;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::ExpBaseNode;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub declares: Vec<DeclareNode>,
}
impl Node {
    pub fn new(tokens: &mut Tokens) -> Node {
        let mut declares: Vec<DeclareNode> = Vec::new();
        declares.push(DeclareNode::new(tokens));
        Node { declares }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeclareNode {
    Function(FunctionNode),
}
impl DeclareNode {
    fn new(tokens: &mut Tokens) -> DeclareNode {
        DeclareNode::Function(FunctionNode::new(tokens))
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
        let _function_type = match tokens.pop() {
            Some(token) => match token {
                Token::Type(function_type) => function_type,
                _ => panic!(),
            },
            None => panic!(),
        };
        let identifier = match tokens.pop() {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        tokens.pop(); // consume ParenS
        let arguments = vec![];
        tokens.pop(); // consume ParenE
        tokens.pop(); // consume BlockS
        let mut statements: Vec<StatementNode> = Vec::new();
        loop {
            match tokens.peek() {
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

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Expression(ExpressionNode),
    Return(ReturnNode),
    Variable(VariableNode),
}
impl StatementNode {
    fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek() {
            Some(token) => match token {
                Token::Return => StatementNode::Return(ReturnNode::new(tokens)),
                Token::Type(_) => StatementNode::Variable(VariableNode::new(tokens)),
                _ => StatementNode::Expression(ExpressionNode::new(tokens)), // TODO: impl error handling
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNode {
    pub expression: ExpBaseNode,
}
impl ExpressionNode {
    fn new(tokens: &mut Tokens) -> ExpressionNode {
        let expression = ExpBaseNode::new(tokens);
        // consume ";"
        let _ = match tokens.peek() {
            Some(token) => match token {
                Token::Semi => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        ExpressionNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
    pub expression: ExpBaseNode,
}
impl ReturnNode {
    fn new(tokens: &mut Tokens) -> ReturnNode {
        // consume "return"
        let _ = match tokens.peek() {
            Some(token) => match token {
                Token::Return => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        let expression = ExpBaseNode::new(tokens);
        // consume ";"
        let _ = match tokens.peek() {
            Some(token) => match token {
                Token::Semi => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        ReturnNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableNode {
    pub identifier: String,
}
impl VariableNode {
    fn new(tokens: &mut Tokens) -> VariableNode {
        let _function_type = match tokens.pop() {
            Some(token) => match token {
                Token::Type(function_type) => function_type,
                _ => panic!(),
            },
            None => panic!(),
        };
        let identifier = match tokens.pop() {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        // consume ";"
        let _ = match tokens.peek() {
            Some(token) => match token {
                Token::Semi => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        VariableNode { identifier }
    }
}
