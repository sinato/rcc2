use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::UnaryNode;
use crate::parser::node::{ExpBaseNode, ExpressionNode};

#[derive(Debug, PartialEq, Clone)]
pub enum VariableNode {
    Direct(DirectDeclareNode),
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
                    VariableNode::Direct(DirectDeclareNode::make(variable_type, tokens))
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
pub enum DirectDeclareNode {
    Simple(SimpleDeclareNode),
    Array(ArrayVariableNode),
}
impl DirectDeclareNode {
    fn make(_variable_type: String, tokens: &mut Tokens) -> DirectDeclareNode {
        let identifier = match tokens.peek(1) {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        match tokens.peek(2) {
            Some(token) => match token {
                Token::Semi => DirectDeclareNode::Simple(SimpleDeclareNode::Simple(
                    SimpleVariableNode::make(_variable_type, tokens),
                )),
                Token::Op(op, _) => match op.as_ref() {
                    "=" => DirectDeclareNode::Simple(SimpleDeclareNode::Initialize(
                        InitializeVariableNode::make(_variable_type, identifier, tokens),
                    )),
                    _ => panic!(),
                },
                Token::SquareS => {
                    DirectDeclareNode::Array(ArrayVariableNode::make(_variable_type, tokens))
                }
                _ => panic!(),
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SimpleDeclareNode {
    Simple(SimpleVariableNode),
    Initialize(InitializeVariableNode),
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

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayVariableNode {
    pub identifier: String,
    pub size: u32,
}
impl ArrayVariableNode {
    fn make(_variable_type: String, tokens: &mut Tokens) -> ArrayVariableNode {
        let identifier = match tokens.pop() {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        if let Some(Token::SquareS) = tokens.pop() {
            ()
        } else {
            panic!()
        }

        let size_node = ExpBaseNode::new(tokens);
        let size = match size_node {
            ExpBaseNode::Unary(node) => match node {
                UnaryNode::Primary(node) => node.get_number_u64() as u32,
                _ => panic!(),
            },
            _ => panic!(),
        };
        if let Some(Token::SquareE) = tokens.pop() {
            ()
        } else {
            panic!()
        }
        if let Some(Token::Semi) = tokens.pop() {
            ()
        } else {
            panic!()
        }
        ArrayVariableNode { identifier, size }
    }
}
