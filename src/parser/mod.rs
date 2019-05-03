pub mod node;

use crate::lexer::token::Tokens;
use crate::parser::node::Node;

pub fn parser(tokens: &mut Tokens) -> Node {
    Node::new(tokens)
}
