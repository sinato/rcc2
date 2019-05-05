use crate::lexer::token::{Associativity, Token, Tokens};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

use crate::emitter::emitter::Emitter;

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    Unary(UnaryNode),
    Binary(BinaryNode),
}
impl ExpressionNode {
    pub fn new(tokens: &mut Tokens) -> ExpressionNode {
        BinaryNode::new(tokens)
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        match self {
            ExpressionNode::Unary(node) => node.emit(emitter),
            ExpressionNode::Binary(node) => node.emit(emitter),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryNode {
    Primary(PrimaryNode),
    Prefix(PrefixNode),
    Suffix(SuffixNode),
}
impl UnaryNode {
    fn new(tokens: &mut Tokens) -> UnaryNode {
        match tokens.peek(0) {
            Some(token) => match token {
                Token::Op(op, _) => match op.as_ref() {
                    "*" | "&" => UnaryNode::new_with_prefix(tokens),
                    _ => UnaryNode::new_with_suffix(tokens),
                },
                _ => UnaryNode::new_with_suffix(tokens),
            },
            None => UnaryNode::new_with_suffix(tokens),
        }
    }
    fn new_with_prefix(tokens: &mut Tokens) -> UnaryNode {
        let (op, _property) = tokens
            .consume_operator()
            .expect("UnaryNode, new_with_prefix");
        match op.as_ref() {
            "*" | "&" => UnaryNode::Prefix(PrefixNode {
                op,
                val: PrimaryNode::new(tokens),
            }),
            _ => panic!(),
        }
    }
    fn new_with_suffix(tokens: &mut Tokens) -> UnaryNode {
        match tokens.peek(1) {
            Some(token) => match token {
                Token::SquareS => UnaryNode::Suffix(SuffixNode::Array(ArrayNode::new(tokens))),
                Token::ParenS => {
                    UnaryNode::Suffix(SuffixNode::FunctionCall(FunctionCallNode::new(tokens)))
                }
                _ => UnaryNode::Primary(PrimaryNode::new(tokens)),
            },
            None => UnaryNode::Primary(PrimaryNode::new(tokens)),
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        match self {
            UnaryNode::Primary(node) => node.emit(emitter),
            UnaryNode::Prefix(node) => node.emit(emitter),
            UnaryNode::Suffix(node) => node.emit(emitter),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixNode {
    pub op: String,
    pub val: PrimaryNode,
}
impl PrefixNode {
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        match self.op.as_ref() {
            "*" => {
                let identifier = self.val.get_identifier();
                let alloca = match emitter.environment.get(&identifier) {
                    Some(alloca) => alloca,
                    None => panic!(format!(
                        "error: use of undeclared identifier \'{}\'",
                        identifier
                    )),
                };
                emitter
                    .builder
                    .build_load(alloca, &identifier)
                    .into_int_value()
            } // dereference
            "&" => {
                let identifier = self.val.get_identifier();
                let alloca = match emitter.environment.get(&identifier) {
                    Some(alloca) => alloca,
                    None => panic!(format!(
                        "error: use of undeclared identifier \'{}\'",
                        identifier
                    )),
                };
                emitter
                    .builder
                    .build_load(alloca, &identifier)
                    .into_int_value()
            } // reference
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SuffixNode {
    Array(ArrayNode),
    FunctionCall(FunctionCallNode),
}
impl SuffixNode {
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        match self {
            SuffixNode::Array(node) => node.emit(emitter),
            SuffixNode::FunctionCall(node) => node.emit(emitter),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayNode {
    pub identifier: String,
    pub indexer: Box<ExpressionNode>,
}
impl ArrayNode {
    fn new(tokens: &mut Tokens) -> ArrayNode {
        let msg = "ArrayNode";
        let identifier = tokens.consume_identifier().expect(msg);
        tokens.consume_square_s().expect(msg);
        let indexer = ExpressionNode::new(tokens);
        tokens.consume_square_e().expect(msg);
        ArrayNode {
            identifier,
            indexer: Box::new(indexer),
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let identifier = self.identifier;
        let array_alloca = match emitter.environment.get(&identifier) {
            Some(alloca) => alloca,
            None => panic!(format!(
                "error: use of undeclared identifier \'{}\'",
                identifier
            )),
        };
        let const_zero: IntValue = emitter.context.i32_type().const_int(0, false);
        let indexer: IntValue = self.indexer.emit(emitter);
        let array_element_alloca = unsafe {
            emitter
                .builder
                .build_gep(array_alloca, &[const_zero, indexer], "extracted_value")
        };
        emitter
            .builder
            .build_load(array_element_alloca, &identifier)
            .into_int_value()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub identifier: String,
    pub parameters: Vec<ExpressionNode>,
}
impl FunctionCallNode {
    fn new(tokens: &mut Tokens) -> FunctionCallNode {
        let msg = "FunctionCallNode";
        let identifier = tokens.consume_identifier().expect(msg);
        tokens.consume_paren_s().expect(msg);
        let mut parameters = vec![];
        while let Some(token) = tokens.peek(0) {
            match token {
                Token::ParenE => break,
                _ => {
                    let parameter = ExpressionNode::new(tokens);
                    parameters.push(parameter);
                    if let Some(Token::Comma) = tokens.peek(0) {
                        tokens.pop();
                    }
                }
            }
        }
        tokens.consume_paren_e().expect(msg);
        FunctionCallNode {
            identifier,
            parameters,
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let identifier = self.identifier;
        let fn_value = match emitter.module.get_function(&identifier) {
            Some(function) => function,
            None => panic!(format!("undefined reference to {:?}", identifier)),
        };
        let parameters: Vec<BasicValueEnum> = self
            .parameters
            .into_iter()
            .map(|val| val.emit(emitter))
            .map(|val| val.into())
            .collect();
        let func_call_site = emitter.builder.build_call(fn_value, &parameters, "call");
        func_call_site
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryNode {
    pub token: Token,
}
impl PrimaryNode {
    fn new(tokens: &mut Tokens) -> PrimaryNode {
        match tokens.pop() {
            Some(token) => match token {
                Token::Num(num_string) => PrimaryNode {
                    token: Token::Num(num_string),
                },
                Token::Ide(ide_string) => PrimaryNode {
                    token: Token::Ide(ide_string),
                },
                _ => panic!(),
            },
            None => panic!(),
        }
    }
    pub fn get_number_u64(&self) -> u64 {
        match self.clone().token {
            Token::Num(num) => num.parse::<u64>().expect(""),
            _ => panic!(),
        }
    }
    pub fn get_identifier(&self) -> String {
        match self.clone().token {
            Token::Ide(identifier) => identifier,
            _ => panic!(),
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        match self.token {
            Token::Num(_) => {
                let num = self.get_number_u64();
                emitter.context.i32_type().const_int(num, false)
            }
            Token::Ide(_) => {
                let identifier = self.get_identifier();
                let alloca = match emitter.environment.get(&identifier) {
                    Some(alloca) => alloca,
                    None => panic!(format!(
                        "error: use of undeclared identifier \'{}\'",
                        identifier
                    )),
                };
                emitter
                    .builder
                    .build_load(alloca, &identifier)
                    .into_int_value()
            }
            _ => panic!(),
        }
    }
}

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

    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        // define main function
        let ret = match self.op {
            Token::Op(op, _) => match op.as_ref() {
                "=" => {
                    // lhs
                    let alloca: PointerValue = match *self.lhs {
                        ExpressionNode::Unary(node) => match node {
                            UnaryNode::Primary(node) => {
                                let identifier = node.get_identifier();
                                match emitter.environment.get(&identifier) {
                                    Some(alloca) => alloca,
                                    None => panic!(),
                                }
                            }
                            UnaryNode::Suffix(node) => match node {
                                SuffixNode::Array(node) => {
                                    let identifier = node.identifier;
                                    let array_alloca = match emitter.environment.get(&identifier) {
                                        Some(alloca) => alloca,
                                        None => panic!(),
                                    };
                                    let const_zero: IntValue =
                                        emitter.context.i32_type().const_int(0, false);
                                    let indexer: IntValue = node.indexer.emit(emitter);
                                    unsafe {
                                        emitter.builder.build_gep(
                                            array_alloca,
                                            &[const_zero, indexer],
                                            "insert",
                                        )
                                    }
                                }
                                SuffixNode::FunctionCall(_node) => {
                                    panic!("need to impl!!!!!!!!!!!!")
                                }
                            },
                            _ => panic!(),
                        },
                        _ => panic!(),
                    };
                    // rhs
                    let val = self.rhs.emit(emitter);
                    emitter.builder.build_store(alloca, val);
                    emitter.context.i32_type().const_int(0, false)
                }
                _ => {
                    let const_lhs = self.lhs.emit(emitter);
                    let const_rhs = self.rhs.emit(emitter);
                    match op.as_ref() {
                        "+" => emitter.builder.build_int_add(const_lhs, const_rhs, "main"),
                        "-" => emitter.builder.build_int_sub(const_lhs, const_rhs, "main"),
                        "*" => emitter.builder.build_int_mul(const_lhs, const_rhs, "main"),
                        "/" => emitter
                            .builder
                            .build_int_unsigned_div(const_lhs, const_rhs, "main"),
                        _ => panic!("Operator not implemented."),
                    }
                }
            },
            _ => panic!(),
        };
        ret
    }
}
