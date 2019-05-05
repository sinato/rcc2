use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

use std::path;

use crate::emitter::environment::Environment;
use crate::lexer::token::Token;
use crate::parser::node::declare::{DeclareNode, DirectDeclareNode};
use crate::parser::node::expression::{
    ArrayNode, BinaryNode, ExpressionNode, FunctionCallNode, PrefixNode, PrimaryNode, SuffixNode,
    UnaryNode,
};
use crate::parser::node::statement::{
    DeclareStatementNode, ExpressionStatementNode, ReturnStatementNode, StatementNode,
};
use crate::parser::node::{FunctionNode, Node, TopLevelDeclareNode};

pub struct Emitter {
    context: Context,
    builder: Builder,
    module: Module,
    environment: Environment,
}
impl Emitter {
    pub fn new() -> Emitter {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("my_module");
        Emitter {
            context,
            builder,
            module,
            environment: Environment::new(),
        }
    }
    pub fn print_to_file(&self) {
        let _ = self.module.print_to_file(path::Path::new("compiled.ll"));
    }
    pub fn emit(&mut self, node: Node) {
        let mut declares = node.declares;
        declares.reverse();
        while let Some(declare) = declares.pop() {
            self.emit_top_level_declare(declare)
        }
    }
    pub fn emit_top_level_declare(&mut self, node: TopLevelDeclareNode) {
        match node {
            TopLevelDeclareNode::Function(node) => self.emit_function(node),
        }
    }
    pub fn emit_function(&mut self, node: FunctionNode) {
        // prepare
        let mut arguments = node.arguments;
        arguments.reverse();
        let mut statements = node.statements.clone();
        statements.reverse();

        let parameters: Vec<BasicTypeEnum> = arguments
            .iter()
            .map(|_| self.context.i32_type().into())
            .collect();
        let function = self.module.add_function(
            &node.identifier,
            self.context.i32_type().fn_type(&parameters, false),
            None,
        );
        let basic_block = self.context.append_basic_block(&function, "entry");
        self.builder.position_at_end(&basic_block);

        for (i, parameter_declare) in arguments.into_iter().enumerate() {
            let parameter_value = match function.get_nth_param(i as u32) {
                Some(val) => val.into_int_value(),
                None => panic!(),
            };
            let parameter_alloca = self
                .builder
                .build_alloca(self.context.i32_type(), &parameter_declare.get_identifier());
            self.builder.build_store(parameter_alloca, parameter_value);
            self.environment
                .update(parameter_declare.get_identifier(), parameter_alloca);
        }

        while let Some(statement) = statements.pop() {
            self.emit_statement(statement);
        }
    }
    pub fn emit_statement(&mut self, node: StatementNode) -> IntValue {
        match node {
            StatementNode::Declare(node) => self.emit_declare_statement(node),
            StatementNode::Return(node) => self.emit_return(node),
            StatementNode::Expression(node) => self.emit_expression_statement(node),
        }
    }
    fn emit_declare_statement(&mut self, node: DeclareStatementNode) -> IntValue {
        self.emit_declare(node.declare)
    }
    fn emit_declare(&mut self, node: DeclareNode) -> IntValue {
        let const_zero = self.context.i32_type().const_int(0, false);
        match node {
            DeclareNode::Direct(node) => match node {
                DirectDeclareNode::Variable(node) => match node.init_expression {
                    Some(expression) => {
                        let identifier = node.identifier;
                        let alloca = self
                            .builder
                            .build_alloca(self.context.i32_type(), &identifier);
                        self.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        self.emit_expression(expression) // Initialize
                    }
                    None => {
                        let identifier = node.identifier;
                        let alloca = self
                            .builder
                            .build_alloca(self.context.i32_type(), &identifier);
                        self.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        const_zero
                    }
                },
                DirectDeclareNode::Array(node) => {
                    let identifier = node.identifier;
                    let array_type = self.context.i32_type().array_type(node.init_size);
                    let alloca = match self.environment.get(&identifier) {
                        Some(_) => panic!(format!("redefinition of {}", identifier)),
                        None => self.builder.build_alloca(array_type, &identifier),
                    };
                    self.environment.update(identifier, alloca);
                    const_zero
                }
            },
            DeclareNode::Pointer(node) => {
                let identifier = node.identifier;
                let alloca = self
                    .builder
                    .build_alloca(self.context.i32_type(), &identifier);
                self.environment.update(identifier, alloca); // TODO: impl detect redefinition
                self.context.i32_type().const_int(0, false)
            }
        }
    }
    pub fn emit_return(&self, node: ReturnStatementNode) -> IntValue {
        let ret = self.emit_expression(node.expression);
        self.builder.build_return(Some(&ret));
        self.context.i32_type().const_int(0, false)
    }

    pub fn emit_expression_statement(&self, node: ExpressionStatementNode) -> IntValue {
        self.emit_expression(node.expression)
    }

    pub fn emit_expression(&self, node: ExpressionNode) -> IntValue {
        match node {
            ExpressionNode::Unary(node) => match node {
                UnaryNode::Primary(node) => self.emit_primary(node),
                UnaryNode::Prefix(node) => self.emit_prefix(node),
                UnaryNode::Suffix(node) => self.emit_suffix(node),
            },
            ExpressionNode::Binary(node) => self.emit_binary(node),
        }
    }

    fn emit_binary(&self, node: BinaryNode) -> IntValue {
        // define main function
        let ret = match node.op {
            Token::Op(op, _) => match op.as_ref() {
                "=" => {
                    // lhs
                    let alloca: PointerValue = match *node.lhs {
                        ExpressionNode::Unary(node) => match node {
                            UnaryNode::Primary(node) => {
                                let identifier = node.get_identifier();
                                match self.environment.get(&identifier) {
                                    Some(alloca) => alloca,
                                    None => panic!(),
                                }
                            }
                            UnaryNode::Suffix(node) => match node {
                                SuffixNode::Array(node) => {
                                    let identifier = node.identifier;
                                    let array_alloca = match self.environment.get(&identifier) {
                                        Some(alloca) => alloca,
                                        None => panic!(),
                                    };
                                    let const_zero: IntValue =
                                        self.context.i32_type().const_int(0, false);
                                    let indexer: IntValue = self.emit_expression(*node.indexer);
                                    unsafe {
                                        self.builder.build_gep(
                                            array_alloca,
                                            &[const_zero, indexer],
                                            "insert",
                                        )
                                    }
                                }
                                SuffixNode::FunctionCall(_node) => {
                                    panic!("need to impl!!!!!!!!!!!!")
                                }
                            },
                            _ => panic!(),
                        },
                        _ => panic!(),
                    };
                    // rhs
                    let val = self.emit_expression(*node.rhs);
                    self.builder.build_store(alloca, val);
                    self.context.i32_type().const_int(0, false)
                }
                _ => {
                    let const_lhs = self.emit_expression(*node.lhs);
                    let const_rhs = self.emit_expression(*node.rhs);
                    match op.as_ref() {
                        "+" => self.builder.build_int_add(const_lhs, const_rhs, "main"),
                        "-" => self.builder.build_int_sub(const_lhs, const_rhs, "main"),
                        "*" => self.builder.build_int_mul(const_lhs, const_rhs, "main"),
                        "/" => self
                            .builder
                            .build_int_unsigned_div(const_lhs, const_rhs, "main"),
                        _ => panic!("Operator not implemented."),
                    }
                }
            },
            _ => panic!(),
        };
        ret
    }
    fn emit_primary(&self, node: PrimaryNode) -> IntValue {
        match node.token {
            Token::Num(_) => {
                let num = node.get_number_u64();
                self.context.i32_type().const_int(num, false)
            }
            Token::Ide(_) => {
                let identifier = node.get_identifier();
                let alloca = match self.environment.get(&identifier) {
                    Some(alloca) => alloca,
                    None => panic!(format!(
                        "error: use of undeclared identifier \'{}\'",
                        identifier
                    )),
                };
                self.builder
                    .build_load(alloca, &identifier)
                    .into_int_value()
            }
            _ => panic!(),
        }
    }
    fn emit_prefix(&self, node: PrefixNode) -> IntValue {
        match node.op.as_ref() {
            "*" => {
                let identifier = node.val.get_identifier();
                let alloca = match self.environment.get(&identifier) {
                    Some(alloca) => alloca,
                    None => panic!(format!(
                        "error: use of undeclared identifier \'{}\'",
                        identifier
                    )),
                };
                self.builder
                    .build_load(alloca, &identifier)
                    .into_int_value()
            } // dereference
            "&" => {
                let identifier = node.val.get_identifier();
                let alloca = match self.environment.get(&identifier) {
                    Some(alloca) => alloca,
                    None => panic!(format!(
                        "error: use of undeclared identifier \'{}\'",
                        identifier
                    )),
                };
                self.builder
                    .build_load(alloca, &identifier)
                    .into_int_value()
            } // reference
            _ => panic!(),
        }
    }
    fn emit_suffix(&self, node: SuffixNode) -> IntValue {
        match node {
            SuffixNode::Array(node) => self.emit_array(node),
            SuffixNode::FunctionCall(node) => self.emit_function_call(node),
        }
    }
    fn emit_array(&self, node: ArrayNode) -> IntValue {
        let identifier = node.identifier;
        let array_alloca = match self.environment.get(&identifier) {
            Some(alloca) => alloca,
            None => panic!(format!(
                "error: use of undeclared identifier \'{}\'",
                identifier
            )),
        };
        let const_zero: IntValue = self.context.i32_type().const_int(0, false);
        let indexer: IntValue = self.emit_expression(*node.indexer);
        let array_element_alloca = unsafe {
            self.builder
                .build_gep(array_alloca, &[const_zero, indexer], "extracted_value")
        };
        self.builder
            .build_load(array_element_alloca, &identifier)
            .into_int_value()
    }
    fn emit_function_call(&self, node: FunctionCallNode) -> IntValue {
        let identifier = node.identifier;
        let fn_value = match self.module.get_function(&identifier) {
            Some(function) => function,
            None => panic!(format!("undefined reference to {:?}", identifier)),
        };
        let parameters: Vec<BasicValueEnum> = node
            .parameters
            .into_iter()
            .map(|val| self.emit_expression(val))
            .map(|val| val.into())
            .collect();
        let func_call_site = self.builder.build_call(fn_value, &parameters, "call");
        func_call_site
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value()
    }
}
