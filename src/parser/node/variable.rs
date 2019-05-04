use crate::lexer::token::{Token, Tokens};
use crate::parser::node::ExpressionNode;

#[derive(Debug, PartialEq, Clone)]
pub enum VariableNode {
    Simple(SimpleDeclareNode),
    Pointer(PointerVariableNode),
}
impl VariableNode {
    pub fn new(tokens: &mut Tokens) -> VariableNode {
        let variable_type = match tokens.pop() {
            Some(token) => match token {
                Token::Type(variable_type) => variable_type,
                _ => panic!(),
            },
            None => panic!(),
        };
        match tokens.peek(1) {
            Some(token) => match token {
                Token::Ide(_identifier) => {
                    VariableNode::Simple(SimpleDeclareNode::make(variable_type, tokens))
                }
                Token::Op(op, _) => match op.as_ref() {
                    "*" => VariableNode::Pointer(PointerVariableNode::make(variable_type, tokens)),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => panic!(),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct PointerVariableNode {
    pub identifier: String,
}
impl PointerVariableNode {
    fn make(_variable_type: String, tokens: &mut Tokens) -> PointerVariableNode {
        tokens.pop(); // consume "*"
        let identifier = match tokens.pop() {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        // consume ";"
        let _ = match tokens.peek(1) {
            Some(token) => match token {
                Token::Semi => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        PointerVariableNode { identifier }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SimpleDeclareNode {
    Simple(SimpleVariableNode),
    Initialize(InitializeVariableNode),
}
impl SimpleDeclareNode {
    fn make(_variable_type: String, tokens: &mut Tokens) -> SimpleDeclareNode {
        let identifier = match tokens.peek(1) {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        match tokens.peek(2) {
            Some(token) => match token {
                Token::Semi => {
                    SimpleDeclareNode::Simple(SimpleVariableNode::make(_variable_type, tokens))
                }
                Token::Op(op, _) => match op.as_ref() {
                    "=" => SimpleDeclareNode::Initialize(InitializeVariableNode::make(
                        _variable_type,
                        identifier,
                        tokens,
                    )),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleVariableNode {
    pub identifier: String,
}
impl SimpleVariableNode {
    fn make(_variable_type: String, tokens: &mut Tokens) -> SimpleVariableNode {
        let identifier = match tokens.pop() {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        // consume ";"
        let _ = match tokens.peek(1) {
            Some(token) => match token {
                Token::Semi => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        SimpleVariableNode { identifier }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InitializeVariableNode {
    pub identifier: String,
    pub expression: ExpressionNode,
}
impl InitializeVariableNode {
    fn make(
        _variable_type: String,
        identifier: String,
        tokens: &mut Tokens,
    ) -> InitializeVariableNode {
        let expression = ExpressionNode::new(tokens);
        InitializeVariableNode {
            identifier,
            expression,
        }
    }
}
