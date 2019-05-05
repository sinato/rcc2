use crate::lexer::token::{Token, Tokens};

#[derive(Debug, PartialEq, Clone)]
pub struct PointerDeclareNode {
    pub identifier: String,
}
impl PointerDeclareNode {
    pub fn new(tokens: &mut Tokens) -> PointerDeclareNode {
        let _variable_type = tokens.consume_type().expect("type");
        tokens.pop(); // consume "*"
        let identifier = match tokens.pop() {
            Some(token) => match token {
                Token::Ide(identifier) => identifier,
                _ => panic!(),
            },
            None => panic!(),
        };
        PointerDeclareNode { identifier }
    }
}
