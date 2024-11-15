use crate::codegen::simple::SimpleCodeGen;

use super::identifier::Identifier;

pub struct Type {
    pub is_struct: bool,
    pub name: Identifier,
    pub pointer_layers: usize,
}

impl SimpleCodeGen for Type {
    fn generate(&self) -> String {
        let mut result: String = String::new();
        if self.is_struct {
            result.push_str("struct ");
        }
        result.push_str(self.name.generate().as_str());
        for _i in 0..self.pointer_layers {
            result.push_str("*");
        }
        result
    }
}
