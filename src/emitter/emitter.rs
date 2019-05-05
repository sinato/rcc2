use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

use std::path;

use crate::emitter::environment::Environment;
use crate::parser::node::Node;

pub struct Emitter {
    pub context: Context,
    pub builder: Builder,
    pub module: Module,
    pub environment: Environment,
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
        node.emit(self)
    }
}
