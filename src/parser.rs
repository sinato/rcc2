use crate::lexer::{Associativity, Token, Tokens};

pub fn parser(tokens: &mut Tokens) -> Node {
    Node::new(tokens)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Function(FunctionNode),
}
impl Node {
    fn new(tokens: &mut Tokens) -> Node {
        Node::Function(FunctionNode::new(tokens))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    identifier: String,
    arguments: Vec<ExpNode>,
    pub expression: ExpNode,
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
        tokens.pop(); // consume return
        let expression = ExpNode::new(tokens);
        tokens.pop(); // consume Semi
        tokens.pop(); // consume BlockE
        FunctionNode {
            identifier,
            arguments,
            expression,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpNode {
    Primary(PrimaryNode),
    Binary(BinaryNode),
}
impl ExpNode {
    fn new(tokens: &mut Tokens) -> ExpNode {
        BinaryNode::new(tokens)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryNode {
    pub token: Token,
}
impl PrimaryNode {
    fn new(tokens: &mut Tokens) -> ExpNode {
        if let Some(Token::Num(num_string)) = tokens.pop() {
            ExpNode::Primary(PrimaryNode {
                token: Token::Num(num_string),
            })
        } else {
            panic!()
        }
    }
    pub fn get_number_u64(&self) -> u64 {
        match self.clone().token {
            Token::Num(num) => num.parse::<u64>().expect(""),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryNode {
    pub op: Token,
    pub lhs: Box<ExpNode>,
    pub rhs: Box<ExpNode>,
}
impl BinaryNode {
    pub fn new(tokens: &mut Tokens) -> ExpNode {
        let lhs = PrimaryNode::new(tokens);
        BinaryNode::binary_expression(lhs, tokens, 0)
    }
    fn binary_expression(mut lhs: ExpNode, tokens: &mut Tokens, min_precedence: u32) -> ExpNode {
        while let Some(token) = tokens.peek() {
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
                    let mut rhs = ExpNode::new(tokens);
                    while let Some(Token::Op(_, property2)) = tokens.peek() {
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
                    lhs = ExpNode::Binary(BinaryNode {
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
