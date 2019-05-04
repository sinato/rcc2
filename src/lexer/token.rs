#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub precedence: u32,
    pub associativity: Associativity,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Type(String), // TODO change to Enum
    ParenS,
    ParenE,
    BlockS,
    BlockE,
    SquareS,
    SquareE,
    Semi,
    Return,
    Num(String),
    Op(String, Property),
    Ide(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
impl Tokens {
    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.reverse();
        let token = self.tokens.pop();
        self.tokens.reverse();
        token
    }
    pub fn peek(&self, num: usize) -> Option<Token> {
        let tokens = self.clone().tokens;
        if num == 0 {
            panic!();
        } else {
            tokens.into_iter().nth(num - 1)
        }
    }
}
