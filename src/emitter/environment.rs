use inkwell::values::PointerValue;

pub struct Environment {
    variables: Vec<(String, PointerValue)>,
}
impl Environment {
    pub fn new() -> Environment {
        let variables: Vec<(String, PointerValue)> = Vec::new();
        Environment { variables }
    }
    pub fn get(&self, skey: &String) -> Option<PointerValue> {
        match self.variables.iter().rev().find(|x| &x.0 == skey) {
            Some(val) => Some(val.1),
            None => None,
        }
    }
    fn find(&self, skey: &String) -> Option<usize> {
        match self.variables.iter().rev().position(|x| &x.0 == skey) {
            Some(idx) => Some(self.variables.len() - idx - 1),
            None => None,
        }
    }
    pub fn update(&mut self, skey: String, sval: PointerValue) {
        match self.find(&skey) {
            Some(idx) => self.variables[idx] = (skey, sval),
            None => self.variables.push((skey, sval)),
        }
    }
}
