use inkwell::values::IntValue;

use crate::emitter::emitter::Emitter;
use crate::emitter::environment::{IntVariable, Variable};
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
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let identifier = self.identifier;
        let alloca = emitter
            .builder
            .build_alloca(emitter.context.i32_type(), &identifier);
        let variable = Variable::Int(IntVariable{ name: identifier.clone(), pointer: alloca});
        emitter.environment.update(identifier, variable); // TODO: impl detect redefinition
        emitter.context.i32_type().const_int(0, false)
    }
}
