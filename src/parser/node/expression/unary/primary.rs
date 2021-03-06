use crate::emitter::emitter::Emitter;
use crate::emitter::environment::{Value, Variable};
use crate::lexer::token::{Token, Tokens};

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryNode {
    pub token: Token,
}
impl PrimaryNode {
    pub fn new(tokens: &mut Tokens) -> PrimaryNode {
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
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        match self.token {
            Token::Num(_) => {
                let num = self.get_number_u64();
                Value::Int(emitter.context.i32_type().const_int(num, false))
            }
            Token::Ide(_) => {
                let identifier = self.get_identifier();
                let alloca = match emitter.environment.get(&identifier) {
                    Some(variable) => match variable {
                        Variable::Int(int_variable) => int_variable.pointer,
                        _ => panic!(),
                    },
                    None => panic!(format!(
                        "error: use of undeclared identifier \'{}\'",
                        identifier
                    )),
                };
                Value::Int(
                    emitter
                        .builder
                        .build_load(alloca, &identifier)
                        .into_int_value(),
                )
            }
            _ => panic!(),
        }
    }
}
