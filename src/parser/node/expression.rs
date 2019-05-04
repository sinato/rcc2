use crate::lexer::token::{Associativity, Token, Tokens};

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
    Array(ArrayNode),
}
impl UnaryNode {
    fn new(tokens: &mut Tokens) -> UnaryNode {
        match tokens.peek(1) {
            Some(token) => match token {
                Token::Op(op, _) => match op.as_ref() {
                    "*" | "&" => UnaryNode::Prefix(PrefixNode::new(tokens)),
                    _ => UnaryNode::new_with_suffix(tokens),
                },
                _ => UnaryNode::new_with_suffix(tokens),
            },
            None => UnaryNode::new_with_suffix(tokens),
        }
    }
    fn new_with_suffix(tokens: &mut Tokens) -> UnaryNode {
        match tokens.peek(2) {
            Some(token) => match token {
                Token::SquareS => {
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
                    let indexer = ExpBaseNode::new(tokens);
                    if let Some(Token::SquareE) = tokens.pop() {
                        ()
                    } else {
                        panic!()
                    }
                    UnaryNode::Array(ArrayNode {
                        identifier,
                        indexer: Box::new(indexer),
                    })
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
impl PrefixNode {
    fn new(tokens: &mut Tokens) -> PrefixNode {
        match tokens.pop() {
            Some(token) => match token {
                Token::Op(op, _) => match op.as_ref() {
                    "*" | "&" => PrefixNode {
                        op,
                        val: PrimaryNode::new(tokens),
                    },
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayNode {
    pub identifier: String,
    pub indexer: Box<ExpBaseNode>,
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
        while let Some(token) = tokens.peek(1) {
            match token {
                Token::Op(op, property) => {
                    let (root_precedence, root_associativity) =
                        (property.clone().precedence, property.clone().associativity);
                    if root_precedence < min_precedence {
                        break;
                    }
                    tokens.pop(); // consume op
                    let op = Token::Op(op, property);
                    // TODO: impl error handling
                    let mut rhs = ExpBaseNode::new(tokens);
                    while let Some(Token::Op(_, property2)) = tokens.peek(1) {
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
