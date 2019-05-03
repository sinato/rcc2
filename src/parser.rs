use crate::lexer::{Associativity, Token, Tokens};

pub fn parser(tokens: &mut Tokens) -> Node {
    binary_expression_parser(tokens)
}

pub fn binary_expression_parser(tokens: &mut Tokens) -> Node {
    let node = BinaryExpNode::new(tokens);
    node
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Primary(PrimaryNode),
    BinaryExp(BinaryExpNode),
}
impl Node {
    fn new(tokens: &mut Tokens) -> Node {
        Node::Primary(PrimaryNode::new(tokens))
    }
    pub fn get_number(&self) -> u64 {
        match self.clone() {
            Node::Primary(node) => match node.token {
                Token::Num(num) => num.parse::<u64>().expect(""),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryNode {
    pub token: Token,
}
impl PrimaryNode {
    fn new(tokens: &mut Tokens) -> PrimaryNode {
        let token = tokens.pop().unwrap();
        PrimaryNode { token }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpNode {
    pub op: Token,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}
impl BinaryExpNode {
    pub fn new(tokens: &mut Tokens) -> Node {
        let lhs = Node::new(tokens);
        BinaryExpNode::binary_expression(lhs, tokens, 0)
    }
    fn binary_expression(mut lhs: Node, tokens: &mut Tokens, min_precedence: u32) -> Node {
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
                    let mut rhs = Node::new(tokens);
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
                        rhs = BinaryExpNode::binary_expression(rhs, tokens, precedence)
                    }
                    lhs = Node::BinaryExp(BinaryExpNode {
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
