use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::unary::UnaryNode;
use crate::parser::node::expression::ExpressionNode;

#[derive(Debug, PartialEq, Clone)]
pub enum DirectDeclareNode {
    Variable(VariableDeclareNode),
    Array(ArrayDeclareNode),
}
impl DirectDeclareNode {
    pub fn new(tokens: &mut Tokens) -> DirectDeclareNode {
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
