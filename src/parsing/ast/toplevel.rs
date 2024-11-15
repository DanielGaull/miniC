use crate::codegen::simple::{IndentCodeGen, SimpleCodeGen};

use super::{enumm::Enum, expression::Expression, function::Function, sstruct::Struct, types::Type};

pub enum TopLevel {
    VarDeclaration {
        typ: Type,
        name: String,
        right: Option<Expression>,
    },
    Import {
        name: String,
        is_lib: bool,
    },
    Function(Function),
    Struct(Struct),
    Enum(Enum),
}
impl SimpleCodeGen for TopLevel {
    fn generate(&self) -> String {
        match self {
            TopLevel::VarDeclaration { typ, name, right } => {
                let mut s = String::new();
                s.push_str(typ.generate().as_str());
                s.push_str(" ");
                s.push_str(name.as_str());
                if let Some(value) = right {
                    s.push_str(" = ");
                    s.push_str(value.generate().as_str());
                }
                s.push_str(";");
                s
            },
            TopLevel::Import { name, is_lib } => {
                let mut s = String::new();
                s.push_str("#include ");
                if *is_lib {
                    s.push_str("<");
                    s.push_str(name.as_str());
                    s.push_str(">");
                } else {
                    s.push_str("\"");
                    s.push_str(name.as_str());
                    s.push_str("\"");
                }
                s
            },
            TopLevel::Function(func) => func.generate(0),
            TopLevel::Struct(struc) => struc.generate(),
            TopLevel::Enum(en) => en.generate(),
        }
    }
}
