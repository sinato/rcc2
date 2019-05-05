use inkwell::values::{BasicValueEnum, IntValue};

use crate::emitter::emitter::Emitter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::ExpressionNode;

#[derive(Debug, PartialEq, Clone)]
pub enum SuffixNode {
    Array(ArrayNode),
    FunctionCall(FunctionCallNode),
}
impl SuffixNode {
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        match self {
            SuffixNode::Array(node) => node.emit(emitter),
            SuffixNode::FunctionCall(node) => node.emit(emitter),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayNode {
    pub identifier: String,
    pub indexer: Box<ExpressionNode>,
}
impl ArrayNode {
    pub fn new(tokens: &mut Tokens) -> ArrayNode {
        let msg = "ArrayNode";
        let identifier = tokens.consume_identifier().expect(msg);
        tokens.consume_square_s().expect(msg);
        let indexer = ExpressionNode::new(tokens);
        tokens.consume_square_e().expect(msg);
        ArrayNode {
            identifier,
            indexer: Box::new(indexer),
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let identifier = self.identifier;
        let array_alloca = match emitter.environment.get(&identifier) {
            Some(alloca) => alloca,
            None => panic!(format!(
                "error: use of undeclared identifier \'{}\'",
                identifier
            )),
        };
        let const_zero: IntValue = emitter.context.i32_type().const_int(0, false);
        let indexer: IntValue = self.indexer.emit(emitter);
        let array_element_alloca = unsafe {
            emitter
                .builder
                .build_gep(array_alloca, &[const_zero, indexer], "extracted_value")
        };
        emitter
            .builder
            .build_load(array_element_alloca, &identifier)
            .into_int_value()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub identifier: String,
    pub parameters: Vec<ExpressionNode>,
}
impl FunctionCallNode {
    pub fn new(tokens: &mut Tokens) -> FunctionCallNode {
        let msg = "FunctionCallNode";
        let identifier = tokens.consume_identifier().expect(msg);
        tokens.consume_paren_s().expect(msg);
        let mut parameters = vec![];
        while let Some(token) = tokens.peek(0) {
            match token {
                Token::ParenE => break,
                _ => {
                    let parameter = ExpressionNode::new(tokens);
                    parameters.push(parameter);
                    if let Some(Token::Comma) = tokens.peek(0) {
                        tokens.pop();
                    }
                }
            }
        }
        tokens.consume_paren_e().expect(msg);
        FunctionCallNode {
            identifier,
            parameters,
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let identifier = self.identifier;
        let fn_value = match emitter.module.get_function(&identifier) {
            Some(function) => function,
            None => panic!(format!("undefined reference to {:?}", identifier)),
        };
        let parameters: Vec<BasicValueEnum> = self
            .parameters
            .into_iter()
            .map(|val| val.emit(emitter))
            .map(|val| val.into())
            .collect();
        let func_call_site = emitter.builder.build_call(fn_value, &parameters, "call");
        func_call_site
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value()
    }
}
