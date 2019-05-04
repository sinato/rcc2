use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{IntValue, PointerValue};

use std::path;

use crate::emitter::environment::Environment;
use crate::lexer::token::Token;
use crate::parser::node::expression::{
    ArrayNode, BinaryNode, ExpBaseNode, PrefixNode, PrimaryNode, UnaryNode,
};
use crate::parser::node::variable::{DirectDeclareNode, SimpleDeclareNode, VariableNode};
use crate::parser::node::{
    DeclareNode, ExpressionNode, FunctionNode, Node, ReturnNode, StatementNode,
};

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
            self.emit_declare(declare)
        }
    }
    pub fn emit_declare(&mut self, node: DeclareNode) {
        match node {
            DeclareNode::Function(node) => self.emit_function(node),
        }
    }
    pub fn emit_function(&mut self, node: FunctionNode) {
        let function =
            self.module
                .add_function("main", self.context.i32_type().fn_type(&[], false), None);
        let basic_block = self.context.append_basic_block(&function, "entry");
        self.builder.position_at_end(&basic_block);
        let mut statements = node.statements.clone();
        statements.reverse();
        let mut ret = self.context.i32_type().const_int(0, false);
        while let Some(statement) = statements.pop() {
            ret = self.emit_statement(statement);
        }
        self.builder.build_return(Some(&ret));
    }
    pub fn emit_statement(&mut self, node: StatementNode) -> IntValue {
        match node {
            StatementNode::Expression(node) => self.emit_expression(node),
            StatementNode::Return(node) => self.emit_return(node),
            StatementNode::Variable(node) => self.emit_variable(node),
        }
    }
    pub fn emit_variable(&mut self, node: VariableNode) -> IntValue {
        let const_zero = self.context.i32_type().const_int(0, false);
        match node {
            VariableNode::Direct(node) => match node {
                DirectDeclareNode::Simple(node) => match node {
                    SimpleDeclareNode::Simple(node) => {
                        let identifier = node.identifier;
                        let alloca = self
                            .builder
                            .build_alloca(self.context.i32_type(), &identifier);
                        self.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        const_zero
                    }
                    SimpleDeclareNode::Initialize(node) => {
                        let identifier = node.identifier;
                        let alloca = self
                            .builder
                            .build_alloca(self.context.i32_type(), &identifier);
                        self.environment.update(identifier, alloca); // TODO: impl detect redefinition
                        self.emit_expression(node.expression) // Initialize
                    }
                },
                DirectDeclareNode::Array(node) => {
                    let identifier = node.identifier;
                    let array_type = self.context.i32_type().array_type(node.size);
                    let alloca = match self.environment.get(&identifier) {
                        Some(_) => panic!(format!("redefinition of {}", identifier)),
                        None => self.builder.build_alloca(array_type, &identifier),
                    };
                    self.environment.update(identifier, alloca);
                    const_zero
                }
            },
            VariableNode::Pointer(node) => {
                let identifier = node.identifier;
                let alloca = self
                    .builder
                    .build_alloca(self.context.i32_type(), &identifier);
                self.environment.update(identifier, alloca); // TODO: impl detect redefinition
                self.context.i32_type().const_int(0, false)
            }
        }
    }
    pub fn emit_return(&self, node: ReturnNode) -> IntValue {
        self.emit_exp_base(node.expression)
    }

    pub fn emit_expression(&self, node: ExpressionNode) -> IntValue {
        self.emit_exp_base(node.expression)
    }

    pub fn emit_exp_base(&self, node: ExpBaseNode) -> IntValue {
        match node {
            ExpBaseNode::Unary(node) => match node {
                UnaryNode::Primary(node) => self.emit_primary(node),
                UnaryNode::Prefix(node) => self.emit_prefix(node),
                UnaryNode::Array(node) => self.emit_array(node),
            },
            ExpBaseNode::Binary(node) => self.emit_binary(node),
        }
    }

    fn emit_binary(&self, node: BinaryNode) -> IntValue {
        // define main function
        let ret = match node.op {
            Token::Op(op, _) => match op.as_ref() {
                "=" => {
                    // lhs
                    let alloca: PointerValue = match *node.lhs {
                        ExpBaseNode::Unary(node) => match node {
                            UnaryNode::Primary(node) => {
                                let identifier = node.get_identifier();
                                match self.environment.get(&identifier) {
                                    Some(alloca) => alloca,
                                    None => panic!(),
                                }
                            }
                            UnaryNode::Array(node) => {
                                let identifier = node.identifier;
                                let array_alloca = match self.environment.get(&identifier) {
                                    Some(alloca) => alloca,
                                    None => panic!(),
                                };
                                let const_zero: IntValue =
                                    self.context.i32_type().const_int(0, false);
                                let indexer: IntValue = self.emit_exp_base(*node.indexer);
                                unsafe {
                                    self.builder.build_gep(
                                        array_alloca,
                                        &[const_zero, indexer],
                                        "insert",
                                    )
                                }
                            }
                            _ => panic!(),
                        },
                        _ => panic!(),
                    };
                    // rhs
                    let val = self.emit_exp_base(*node.rhs);
                    self.builder.build_store(alloca, val);
                    self.context.i32_type().const_int(0, false)
                }
                _ => {
                    let const_lhs = self.emit_exp_base(*node.lhs);
                    let const_rhs = self.emit_exp_base(*node.rhs);
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
        let indexer: IntValue = self.emit_exp_base(*node.indexer);
        let array_element_alloca = unsafe {
            self.builder
                .build_gep(array_alloca, &[const_zero, indexer], "extracted_value")
        };
        self.builder
            .build_load(array_element_alloca, &identifier)
            .into_int_value()
    }
}
