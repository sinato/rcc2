use inkwell::values::{IntValue, PointerValue};

pub struct Environment {
    variables: Vec<(String, Variable)>,
}
impl Environment {
    pub fn new() -> Environment {
        let variables: Vec<(String, Variable)> = Vec::new();
        Environment { variables }
    }
    pub fn get(&self, skey: &String) -> Option<Variable> {
        match self.variables.iter().rev().find(|x| &x.0 == skey) {
            Some(val) => Some(val.1.clone()),
            None => None,
        }
    }
    fn find(&self, skey: &String) -> Option<usize> {
        match self.variables.iter().rev().position(|x| &x.0 == skey) {
            Some(idx) => Some(self.variables.len() - idx - 1),
            None => None,
        }
    }
    pub fn update(&mut self, skey: String, sval: Variable) {
        match self.find(&skey) {
            Some(idx) => self.variables[idx] = (skey, sval),
            None => self.variables.push((skey, sval)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Variable {
    Int(IntVariable),
    Array(ArrayVariable),
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntVariable {
    pub name: String,
    pub pointer: PointerValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayVariable {
    pub name: String,
    pub pointer: PointerValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(IntValue),
    Pointer(PointerValue),
    Null,
}
impl Value {
    pub fn get_pointer(self) -> Result<PointerValue, String> {
        match self {
            Value::Pointer(pointer) => Ok(pointer),
            _ => Err("this is not a pointer value".to_string()),
        }
    }
    pub fn get_int(self) -> Result<IntValue, String> {
        match self {
            Value::Int(value) => Ok(value),
            _ => Err("this is not a value value".to_string()),
        }
    }
}
