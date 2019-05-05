use inkwell::values::IntValue;

use crate::emitter::emitter::Emitter;
use crate::lexer::token::{Token, Tokens};
use crate::parser::node::declare::DeclareNode;
use crate::parser::node::expression::ExpressionNode;

#[derive(Debug, PartialEq, Clone)]
pub struct StatementsNode {
    pub statements: Vec<StatementNode>,
}
impl StatementsNode {
    pub fn new(tokens: &mut Tokens) -> StatementsNode {
        let mut statements: Vec<StatementNode> = Vec::new();
        while let Some(token) = tokens.peek(0) {
            match token {
                Token::BlockE => break,
                _ => {
                    let statement = StatementNode::new(tokens);
                    statements.push(statement);
                }
            }
        }
        StatementsNode { statements }
    }
    pub fn emit(self, emitter: &mut Emitter) {
        let mut statements = self.statements.clone();
        statements.reverse();
        while let Some(statement) = statements.pop() {
            statement.emit(emitter);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Declare(DeclareStatementNode),
    Expression(ExpressionStatementNode),
    Return(ReturnStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek(0) {
            Some(token) => match token {
                Token::Return => StatementNode::Return(ReturnStatementNode::new(tokens)),
                Token::Type(_) => StatementNode::Declare(DeclareStatementNode::new(tokens)),
                _ => StatementNode::Expression(ExpressionStatementNode::new(tokens)),
            },
            None => panic!(),
        }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        match self {
            StatementNode::Declare(node) => node.emit(emitter),
            StatementNode::Return(node) => node.emit(emitter),
            StatementNode::Expression(node) => node.emit(emitter),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareStatementNode {
    pub declare: DeclareNode,
}
impl DeclareStatementNode {
    fn new(tokens: &mut Tokens) -> DeclareStatementNode {
        let msg = "ReturnStatementNode";
        let declare = DeclareNode::new(tokens);
        tokens.consume_semi().expect(msg);
        DeclareStatementNode { declare }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        self.declare.emit(emitter)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatementNode {
    pub expression: ExpressionNode,
}
impl ExpressionStatementNode {
    fn new(tokens: &mut Tokens) -> ExpressionStatementNode {
        let msg = "ExpressiontatementNode";
        let expression = ExpressionNode::new(tokens);
        tokens.consume_semi().expect(msg);
        ExpressionStatementNode { expression }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        self.expression.emit(emitter)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatementNode {
    pub expression: ExpressionNode,
}
impl ReturnStatementNode {
    fn new(tokens: &mut Tokens) -> ReturnStatementNode {
        let msg = "ReturnStatementNode";
        tokens.consume_return().expect(msg);
        let expression = ExpressionNode::new(tokens);
        tokens.consume_semi().expect(msg);
        ReturnStatementNode { expression }
    }
    pub fn emit(self, emitter: &mut Emitter) -> IntValue {
        let ret = self.expression.emit(emitter);
        emitter.builder.build_return(Some(&ret));
        emitter.context.i32_type().const_int(0, false)
    }
}
