use crate::emitter::emitter::Emitter;
use crate::emitter::environment::{Value, Variable};
use crate::parser::node::expression::unary::primary::PrimaryNode;

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixNode {
    pub op: String,
    pub val: PrimaryNode,
}
impl PrefixNode {
    pub fn emit(self, emitter: &mut Emitter) -> Value {
        match self.op.as_ref() {
            "*" => {
                let identifier = self.val.get_identifier();
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
            } // dereference
            "&" => {
                let identifier = self.val.get_identifier();
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
            } // reference
            _ => panic!(),
        }
    }
}
