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
    arguments: Vec<ExpBaseNode>, // TODO: change to Vec<declare>
    pub statements: Vec<StatementNode>,
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
        let mut statements: Vec<StatementNode> = Vec::new();
        loop {
            match tokens.peek() {
                Some(token) => match token {
                    Token::BlockE => break,
                    _ => {
                        let statement = StatementNode::new(tokens);
                        statements.push(statement);
                    }
                },
                None => panic!(),
            }
        }
        tokens.pop(); // consume BlockE
        FunctionNode {
            identifier,
            arguments,
            statements,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Expression(ExpressionNode),
    Return(ReturnNode),
}
impl StatementNode {
    fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek() {
            Some(token) => match token {
                Token::Return => StatementNode::Return(ReturnNode::new(tokens)),
                _ => StatementNode::Expression(ExpressionNode::new(tokens)), // TODO: impl error handling
            },
            None => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNode {
    pub expression: ExpBaseNode,
}
impl ExpressionNode {
    fn new(tokens: &mut Tokens) -> ExpressionNode {
        let expression = ExpBaseNode::new(tokens);
        // consume ";"
        let _ = match tokens.peek() {
            Some(token) => match token {
                Token::Semi => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        ExpressionNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
    pub expression: ExpBaseNode,
}
impl ReturnNode {
    fn new(tokens: &mut Tokens) -> ReturnNode {
        // consume "return"
        let _ = match tokens.peek() {
            Some(token) => match token {
                Token::Return => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        let expression = ExpBaseNode::new(tokens);
        // consume ";"
        let _ = match tokens.peek() {
            Some(token) => match token {
                Token::Semi => tokens.pop(),
                _ => panic!(),
            },
            None => panic!(),
        };
        ReturnNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpBaseNode {
    Primary(PrimaryNode),
    Binary(BinaryNode),
}
impl ExpBaseNode {
    fn new(tokens: &mut Tokens) -> ExpBaseNode {
        BinaryNode::new(tokens)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryNode {
    pub token: Token,
}
impl PrimaryNode {
    fn new(tokens: &mut Tokens) -> ExpBaseNode {
        if let Some(Token::Num(num_string)) = tokens.pop() {
            ExpBaseNode::Primary(PrimaryNode {
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
    pub lhs: Box<ExpBaseNode>,
    pub rhs: Box<ExpBaseNode>,
}
impl BinaryNode {
    pub fn new(tokens: &mut Tokens) -> ExpBaseNode {
        let lhs = PrimaryNode::new(tokens);
        BinaryNode::binary_expression(lhs, tokens, 0)
    }
    fn binary_expression(
        mut lhs: ExpBaseNode,
        tokens: &mut Tokens,
        min_precedence: u32,
    ) -> ExpBaseNode {
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
                    let mut rhs = ExpBaseNode::new(tokens);
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
