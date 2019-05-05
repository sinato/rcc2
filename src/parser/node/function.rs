use inkwell::types::BasicTypeEnum;

use crate::emitter::emitter::Emitter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::declare::DeclareNode;
use crate::parser::node::statement::StatementsNode;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub identifier: String,
    pub arguments: Vec<DeclareNode>,
    pub statements: StatementsNode,
}
impl FunctionNode {
    pub fn new(tokens: &mut Tokens) -> FunctionNode {
        let msg = "FunctionNode";
        let _function_type = tokens.consume_type().expect("type");
        let identifier = tokens.consume_identifier().expect("identifier");
        tokens.consume_paren_s().expect(msg);
        let mut arguments = vec![];
        while let Some(Token::Type(_)) = tokens.peek(0) {
            let argument = DeclareNode::new(tokens);
            arguments.push(argument);
            if let Some(Token::Comma) = tokens.peek(0) {
                tokens.pop();
            }
        }
        tokens.consume_paren_e().expect(msg);
        tokens.consume_block_s().expect(msg);
        let statements = StatementsNode::new(tokens);
        tokens.consume_block_e().expect(msg);
        FunctionNode {
            identifier,
            arguments,
            statements,
        }
    }
    pub fn emit(self, emitter: &mut Emitter) {
        // prepare
        let mut arguments = self.arguments;
        arguments.reverse();

        let parameters: Vec<BasicTypeEnum> = arguments
            .iter()
            .map(|_| emitter.context.i32_type().into())
            .collect();
        let function = emitter.module.add_function(
            &self.identifier,
            emitter.context.i32_type().fn_type(&parameters, false),
            None,
        );
        let basic_block = emitter.context.append_basic_block(&function, "entry");
        emitter.builder.position_at_end(&basic_block);

        for (i, parameter_declare) in arguments.into_iter().enumerate() {
            let parameter_value = match function.get_nth_param(i as u32) {
                Some(val) => val.into_int_value(),
                None => panic!(),
            };
            let parameter_alloca = emitter.builder.build_alloca(
                emitter.context.i32_type(),
                &parameter_declare.get_identifier(),
            );
            emitter
                .builder
                .build_store(parameter_alloca, parameter_value);
            emitter
                .environment
                .update(parameter_declare.get_identifier(), parameter_alloca);
        }

        self.statements.emit(emitter);
    }
}
