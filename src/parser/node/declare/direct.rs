use crate::emitter::emitter::Emitter;
use crate::emitter::environment::{ArrayVariable, IntVariable, Value, Variable};
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
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        match self {
            DirectDeclareNode::Variable(node) => node.emit(emitter),
            DirectDeclareNode::Array(node) => node.emit(emitter),
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
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        match self.init_expression {
            Some(expression) => {
                let identifier = self.identifier;
                let alloca = emitter
                    .builder
                    .build_alloca(emitter.context.i32_type(), &identifier);
                let variable = Variable::Int(IntVariable {
                    name: identifier.clone(),
                    pointer: alloca,
                });
                emitter.environment.update(identifier, variable); // TODO: impl detect redefinition
                expression.emit(emitter)
            }
            None => {
                let identifier = self.identifier;
                let alloca = emitter
                    .builder
                    .build_alloca(emitter.context.i32_type(), &identifier);
                let variable = Variable::Int(IntVariable {
                    name: identifier.clone(),
                    pointer: alloca,
                });
                emitter.environment.update(identifier, variable); // TODO: impl detect redefinition
                Value::Null
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayDeclareNode {
    pub identifier: String,
    pub init_sizes: Vec<u32>,
}
impl ArrayDeclareNode {
    fn new(tokens: &mut Tokens) -> ArrayDeclareNode {
        let _variable_type = tokens.consume_type().expect("type");
        let identifier = tokens.consume_identifier().expect("identifier");
        let mut init_sizes = Vec::new();
        while let Some(Token::SquareS) = tokens.peek(0) {
            tokens.consume_square_s().expect("[");
            let size_node = ExpressionNode::new(tokens);
            let init_size = match size_node {
                ExpressionNode::Unary(node) => match node {
                    UnaryNode::Primary(node) => node.get_number_u64() as u32,
                    _ => panic!(),
                },
                _ => panic!(),
            };
            init_sizes.push(init_size);
            tokens.consume_square_e().expect("]");
        }
        ArrayDeclareNode {
            identifier,
            init_sizes,
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        let identifier = self.identifier;

        let mut init_sizes = self.init_sizes;
        // init_sizes.reverse();
        let mut array_type = match init_sizes.pop() {
            Some(init_size) => emitter.context.i32_type().array_type(init_size),
            None => panic!(),
        };
        while let Some(init_size) = init_sizes.pop() {
            array_type = array_type.array_type(init_size);
        }

        let alloca = match emitter.environment.get(&identifier) {
            Some(_) => panic!(format!("redefinition of {}", identifier)),
            None => emitter.builder.build_alloca(array_type, &identifier),
        };
        let variable = Variable::Array(ArrayVariable {
            name: identifier.clone(),
            pointer: alloca,
        });
        emitter.environment.update(identifier, variable);
        Value::Null
    }
}
