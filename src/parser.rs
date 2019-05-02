use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryNode {
    pub token: Token,
}
impl PrimaryNode {
    pub fn get_number(&self) -> u64 {
        let node = self.clone();
        match node.token {
            Token::Num(num) => num.parse::<u64>().expect(""),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpNode {
    pub op: Token,
    pub lhs: PrimaryNode,
    pub rhs: PrimaryNode,
}
