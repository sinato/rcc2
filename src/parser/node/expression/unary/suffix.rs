use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

use crate::emitter::emitter::Emitter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::expression::ExpressionNode;

#[derive(Debug, PartialEq, Clone)]
pub enum SuffixNode {
    Array(ArrayAccessNode),
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
pub struct ArrayElementNode {
    pub identifier: String,
    pub indexer_nodes: Vec<Box<ExpressionNode>>,
}
impl ArrayElementNode {
    pub fn new(tokens: &mut Tokens) -> ArrayElementNode {
        let msg = "ArrayNode";
        let identifier = tokens.consume_identifier().expect(msg);
        let mut indexer_nodes = Vec::new();
        while let Some(Token::SquareS) = tokens.peek(0) {
            tokens.consume_square_s().expect(msg);
            let indexer_node = Box::new(ExpressionNode::new(tokens));
            indexer_nodes.push(indexer_node);
            tokens.consume_square_e().expect(msg);
        }
        ArrayElementNode {
            identifier,
            indexer_nodes,
        }
    }
    pub fn emit_pointer(self, emitter: &mut Emitter) -> PointerValue {
        let identifier = self.identifier;
        let array_alloca = match emitter.environment.get(&identifier) {
            Some(alloca) => alloca,
            None => panic!(format!(
                "error: use of undeclared identifier \'{}\'",
                identifier
            )),
        };
        let const_zero: IntValue = emitter.context.i32_type().const_int(0, false);

        let mut indexer_nodes = self.indexer_nodes;
        indexer_nodes.reverse();
        let mut element_pointer = match indexer_nodes.pop() {
            Some(indexer_node) => {
                let indexer = indexer_node.emit(emitter);
                unsafe {
                    emitter
                        .builder
                        .build_gep(array_alloca, &[const_zero, indexer], "first_element")
                }
            }
            None => panic!(),
        };
        while let Some(indexer_node) = indexer_nodes.pop() {
            let indexer = indexer_node.emit(emitter);
            element_pointer = unsafe {
                emitter
                    .builder
                    .build_gep(element_pointer, &[const_zero, indexer], "element")
            }
        }
        element_pointer
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayAccessNode {
    pub array_element: ArrayElementNode,
}
impl ArrayAccessNode {
    pub fn new(tokens: &mut Tokens) -> ArrayAccessNode {
        let array_element = ArrayElementNode::new(tokens);
        ArrayAccessNode { array_element }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let array_element_alloca = self.array_element.emit_pointer(emitter);
        emitter
            .builder
            .build_load(array_element_alloca, "array_element")
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
