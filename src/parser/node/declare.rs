use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::{ExpressionNode, UnaryNode};
use inkwell::values::IntValue;

use crate::emitter::emitter::Emitter;

#[derive(Debug, PartialEq, Clone)]
pub enum DeclareNode {
    Direct(DirectDeclareNode),
    Pointer(PointerDeclareNode),
}
impl DeclareNode {
    pub fn new(tokens: &mut Tokens) -> DeclareNode {
        // expect Token::Type(_) as tokens.peek(0)
        match tokens.peek(1) {
            Some(token) => match token {
                Token::Ide(_identifier) => DeclareNode::Direct(DirectDeclareNode::new(tokens)),
                Token::Op(op, _) => match op.as_ref() {
                    "*" => DeclareNode::Pointer(PointerDeclareNode::new(tokens)),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => panic!(),
        }
    }
    pub fn get_identifier(&self) -> String {
        match self.clone() {
            DeclareNode::Direct(node) => match node {
                DirectDeclareNode::Variable(node) => node.identifier,
                DirectDeclareNode::Array(node) => node.identifier,
            },
            DeclareNode::Pointer(node) => node.identifier,
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let const_zero = emitter.context.i32_type().const_int(0, false);
        match self {
            DeclareNode::Direct(node) => match node {
                DirectDeclareNode::Variable(node) => match node.init_expression {
                    Some(expression) => {
                        let identifier = node.identifier;
                        let alloca = emitter
                            .builder
                            .build_alloca(emitter.context.i32_type(), &identifier);
                        emitter.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        expression.emit(emitter)
                    }
                    None => {
                        let identifier = node.identifier;
                        let alloca = emitter
                            .builder
                            .build_alloca(emitter.context.i32_type(), &identifier);
                        emitter.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        const_zero
                    }
                },
                DirectDeclareNode::Array(node) => {
                    let identifier = node.identifier;
                    let array_type = emitter.context.i32_type().array_type(node.init_size);
                    let alloca = match emitter.environment.get(&identifier) {
                        Some(_) => panic!(format!("redefinition of {}", identifier)),
                        None => emitter.builder.build_alloca(array_type, &identifier),
                    };
                    emitter.environment.update(identifier, alloca);
                    const_zero
                }
            },
            DeclareNode::Pointer(node) => {
                let identifier = node.identifier;
                let alloca = emitter
                    .builder
                    .build_alloca(emitter.context.i32_type(), &identifier);
                emitter.environment.update(identifier, alloca); // TODO: impl detect redefinition
                emitter.context.i32_type().const_int(0, false)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PointerDeclareNode {
    pub identifier: String,
}
impl PointerDeclareNode {
    fn new(tokens: &mut Tokens) -> PointerDeclareNode {
        let _variable_type = tokens.consume_type().expect("type");
        tokens.pop(); // consume "*"
        let identifier = match tokens.pop() {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        PointerDeclareNode { identifier }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DirectDeclareNode {
    Variable(VariableDeclareNode),
    Array(ArrayDeclareNode),
}
impl DirectDeclareNode {
    fn new(tokens: &mut Tokens) -> DirectDeclareNode {
        // expect Token::Type(_) as tokens.peek(0)
        // expect Token::Ide(_) as tokens.peek(1)
        match tokens.peek(2) {
            Some(token) => match token {
                Token::SquareS => DirectDeclareNode::Array(ArrayDeclareNode::new(tokens)),
                _ => DirectDeclareNode::Variable(VariableDeclareNode::new(tokens)),
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclareNode {
    pub identifier: String,
    pub init_expression: Option<ExpressionNode>,
}
impl VariableDeclareNode {
    fn new(tokens: &mut Tokens) -> VariableDeclareNode {
        let _variable_type = tokens.consume_type().expect("type");
        let identifier = tokens.expect_identifier().expect("identifier");
        match tokens.peek(1) {
            Some(token) => match token {
                Token::Op(op, ..) => match op.as_ref() {
                    "=" => {
                        let init_expression = Some(ExpressionNode::new(tokens));
                        VariableDeclareNode {
                            identifier,
                            init_expression,
                        }
                    }
                    _ => panic!(),
                },
                _ => {
                    let identifier = tokens.consume_identifier().expect("identifier");
                    VariableDeclareNode {
                        identifier,
                        init_expression: None,
                    }
                }
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayDeclareNode {
    pub identifier: String,
    pub init_size: u32,
}
impl ArrayDeclareNode {
    fn new(tokens: &mut Tokens) -> ArrayDeclareNode {
        let _variable_type = tokens.consume_type().expect("type");
        let identifier = tokens.consume_identifier().expect("identifier");
        tokens.consume_square_s().expect("[");
        let size_node = ExpressionNode::new(tokens);
        let init_size = match size_node {
            ExpressionNode::Unary(node) => match node {
                UnaryNode::Primary(node) => node.get_number_u64() as u32,
                _ => panic!(),
            },
            _ => panic!(),
        };
        tokens.consume_square_e().expect("]");
        ArrayDeclareNode {
            identifier,
            init_size,
        }
    }
}
