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
    Comma,
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
        tokens.into_iter().nth(num)
    }
    // TODO: simplify with using macro.
    pub fn expect_identifier(&mut self) -> Result<String, String> {
        if let Some(Token::Ide(identifier)) = self.peek(0) {
            return Ok(identifier);
        }
        return Err("Token::Ide not found".to_string());
    }
    pub fn consume_identifier(&mut self) -> Result<String, String> {
        if let Some(Token::Ide(identifier)) = self.peek(0) {
            self.pop(); // consume
            return Ok(identifier);
        }
        return Err("Token::Ide not found".to_string());
    }
    pub fn consume_operator(&mut self) -> Result<(String, Property), String> {
        if let Some(Token::Op(op, property)) = self.peek(0) {
            self.pop(); // consume
            return Ok((op, property));
        }
        return Err("Token::Ope not found".to_string());
    }
    pub fn consume_semi(&mut self) -> Result<Token, String> {
        if let Some(Token::Semi) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::Semi);
        }
        return Err("Token::Semi not found".to_string());
    }
    pub fn consume_block_s(&mut self) -> Result<Token, String> {
        if let Some(Token::BlockS) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::BlockS);
        }
        return Err("Token::BlockS not found".to_string());
    }
    pub fn consume_block_e(&mut self) -> Result<Token, String> {
        if let Some(Token::BlockE) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::BlockE);
        }
        return Err("Token::BlockE not found".to_string());
    }
    pub fn consume_paren_s(&mut self) -> Result<Token, String> {
        if let Some(Token::ParenS) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::ParenS);
        }
        return Err("Token::ParenS not found".to_string());
    }
    pub fn consume_paren_e(&mut self) -> Result<Token, String> {
        if let Some(Token::ParenE) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::ParenE);
        }
        return Err("Token::ParenE not found".to_string());
    }
    pub fn consume_square_s(&mut self) -> Result<Token, String> {
        if let Some(Token::SquareS) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::SquareS);
        }
        return Err("Token::SquareS not found".to_string());
    }
    pub fn consume_square_e(&mut self) -> Result<Token, String> {
        if let Some(Token::SquareE) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::SquareE);
        }
        return Err("Token::SquareE not found".to_string());
    }
    pub fn consume_type(&mut self) -> Result<String, String> {
        if let Some(Token::Type(typ)) = self.peek(0) {
            self.pop(); // consume
            return Ok(typ);
        }
        return Err("Token::Type not found".to_string());
    }
    pub fn consume_return(&mut self) -> Result<Token, String> {
        if let Some(Token::Return) = self.peek(0) {
            self.pop(); // consume
            return Ok(Token::Return);
        }
        return Err("Token::Return not found".to_string());
    }
}
