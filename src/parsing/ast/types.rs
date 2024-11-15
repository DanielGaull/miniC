use crate::codegen::simple::SimpleCodeGen;

use super::identifier::Identifier;

pub enum TypeType {
    Simple,
    Struct,
    Enum,
    Union,
}

pub struct Type {
    pub typetype: TypeType,
    pub name: Identifier,
    pub pointer_layers: usize,
}

impl SimpleCodeGen for Type {
    fn generate(&self) -> String {
        let mut result: String = String::new();
        match self.typetype {
            TypeType::Simple => (),
            TypeType::Struct => result.push_str("struct "),
            TypeType::Enum => result.push_str("enum "),
            TypeType::Union => result.push_str("union "),
        };
        result.push_str(self.name.generate().as_str());
        for _i in 0..self.pointer_layers {
            result.push_str("*");
        }
        result
    }
}
