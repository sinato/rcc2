use crate::lexer::token::{Associativity, Token, Tokens};

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNode {
    pub expression: ExpBaseNode,
}
impl ExpressionNode {
    pub fn new(tokens: &mut Tokens) -> ExpressionNode {
        let expression = ExpBaseNode::new(tokens);
        tokens.consume_semi().expect("ExpressionNode");
        ExpressionNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpBaseNode {
    Unary(UnaryNode),
    Binary(BinaryNode),
}
impl ExpBaseNode {
    pub fn new(tokens: &mut Tokens) -> ExpBaseNode {
        BinaryNode::new(tokens)
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
        let msg = "UnaryNode, new_with_suffix";
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixNode {
    pub op: String,
    pub val: PrimaryNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SuffixNode {
    Array(ArrayNode),
    FunctionCall(FunctionCallNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayNode {
    pub identifier: String,
    pub indexer: Box<ExpBaseNode>,
}
impl ArrayNode {
    fn new(tokens: &mut Tokens) -> ArrayNode {
        let msg = "ArrayNode";
        let identifier = tokens.consume_identifier().expect(msg);
        tokens.consume_square_s().expect(msg);
        let indexer = ExpBaseNode::new(tokens);
        tokens.consume_square_e().expect(msg);
        ArrayNode {
            identifier,
            indexer: Box::new(indexer),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub identifier: String,
    pub parameters: Vec<Box<ExpBaseNode>>,
}
impl FunctionCallNode {
    fn new(tokens: &mut Tokens) -> FunctionCallNode {
        let msg = "FunctionCallNode";
        let identifier = tokens.consume_identifier().expect(msg);
        tokens.pop(); //consume parenS
        let parameters = vec![];
        tokens.pop(); //consume parenE
        FunctionCallNode {
            identifier,
            parameters,
        }
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryNode {
    pub op: Token,
    pub lhs: Box<ExpBaseNode>,
    pub rhs: Box<ExpBaseNode>,
}
impl BinaryNode {
    pub fn new(tokens: &mut Tokens) -> ExpBaseNode {
        let lhs = ExpBaseNode::Unary(UnaryNode::new(tokens));
        BinaryNode::binary_expression(lhs, tokens, 0)
    }
    fn binary_expression(
        mut lhs: ExpBaseNode,
        tokens: &mut Tokens,
        min_precedence: u32,
    ) -> ExpBaseNode {
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
                    let mut rhs = ExpBaseNode::new(tokens);
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
                    lhs = ExpBaseNode::Binary(BinaryNode {
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
}
