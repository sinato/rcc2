use crate::emitter::emitter::Emitter;
use crate::emitter::environment::{Value, Variable};
use crate::lexer::token::{Associativity, Token, Tokens};
use crate::parser::node::expression::unary::suffix::SuffixNode;
use crate::parser::node::expression::unary::UnaryNode;
use crate::parser::node::expression::ExpressionNode;

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryNode {
    pub op: Token,
    pub lhs: Box<ExpressionNode>,
    pub rhs: Box<ExpressionNode>,
}
impl BinaryNode {
    pub fn new(tokens: &mut Tokens) -> ExpressionNode {
        let lhs = ExpressionNode::Unary(UnaryNode::new(tokens));
        BinaryNode::binary_expression(lhs, tokens, 0)
    }
    fn binary_expression(
        mut lhs: ExpressionNode,
        tokens: &mut Tokens,
        min_precedence: u32,
    ) -> ExpressionNode {
        while let Some(token) = tokens.peek(0) {
            match token {
                Token::Op(_op, property) => {
                    let (root_precedence, root_associativity) =
                        (property.clone().precedence, property.clone().associativity);
                    if root_precedence < min_precedence {
                        break;
                    }
                    let (op, property) = tokens
                        .consume_operator()
                        .expect("BinaryNode, binary_expression");
                    let op = Token::Op(op, property);
                    let mut rhs = ExpressionNode::new(tokens);
                    while let Some(Token::Op(_, property2)) = tokens.peek(0) {
                        let (precedence, _associativity) =
                            (property2.precedence, property2.associativity);
                        match root_associativity {
                            Associativity::Right => {
                                if root_precedence > precedence {
                                    break;
                                }
                            }
                            Associativity::Left => {
                                if root_precedence >= precedence {
                                    break;
                                }
                            }
                        }
                        rhs = BinaryNode::binary_expression(rhs, tokens, precedence)
                    }
                    lhs = ExpressionNode::Binary(BinaryNode {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    });
                }
                _ => break,
            }
        }
        lhs
    }
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        // define main function
        let ret = match self.op {
            Token::Op(op, _) => match op.as_ref() {
                "=" => {
                    // lhs
                    let alloca = match *self.lhs {
                        ExpressionNode::Unary(node) => match node {
                            UnaryNode::Primary(node) => {
                                let identifier = node.get_identifier();
                                match emitter.environment.get(&identifier) {
                                    Some(variable) => match variable {
                                        Variable::Int(int_variable) => int_variable.pointer,
                                        Variable::Array(array_variable) => array_variable.pointer,
                                        Variable::Null => panic!(),
                                    },
                                    None => panic!(),
                                }
                            }
                            UnaryNode::Suffix(node) => match node {
                                SuffixNode::Array(node) => node.array_element.emit_pointer(emitter),
                                SuffixNode::FunctionCall(_node) => {
                                    panic!("need to impl!!!!!!!!!!!!")
                                }
                            },
                            _ => panic!(),
                        },
                        _ => panic!(),
                    };
                    // rhs
                    let val = match self.rhs.emit(emitter).get_int() {
                        Ok(value) => value,
                        Err(msg) => panic!(msg),
                    };
                    emitter.builder.build_store(alloca, val);
                    Value::Null
                }
                _ => {
                    let const_lhs = self.lhs.emit(emitter);
                    let const_lhs = match const_lhs.get_int() {
                        Ok(value) => value,
                        Err(msg) => panic!(msg),
                    };
                    let const_rhs = self.rhs.emit(emitter);
                    let const_rhs = match const_rhs.get_int() {
                        Ok(value) => value,
                        Err(msg) => panic!(msg),
                    };
                    let ret_int_val = match op.as_ref() {
                        "+" => emitter.builder.build_int_add(const_lhs, const_rhs, "main"),
                        "-" => emitter.builder.build_int_sub(const_lhs, const_rhs, "main"),
                        "*" => emitter.builder.build_int_mul(const_lhs, const_rhs, "main"),
                        "/" => emitter
                            .builder
                            .build_int_unsigned_div(const_lhs, const_rhs, "main"),
                        _ => panic!("Operator not implemented."),
                    };
                    Value::Int(ret_int_val)
                }
            },
            _ => panic!(),
        };
        ret
    }
}
