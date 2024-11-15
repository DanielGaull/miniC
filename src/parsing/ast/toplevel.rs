use crate::codegen::simple::{ModuleMemberCodeGen, SimpleCodeGen};

use super::{enumm::Enum, expression::Expression, function::{Function, FunctionHeader}, sstruct::Struct, types::Type};

pub enum TopLevel {
    VarDeclaration {
        typ: Type,
        name: String,
        right: Option<Expression>,
        modifier: Vec<String>,
    },
    Import {
        name: String,
        is_lib: bool,
    },
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    FunctionHeader(FunctionHeader),
}
impl ModuleMemberCodeGen for TopLevel {
    fn generate(&self, name_prefix: String) -> String {
        match self {
            TopLevel::VarDeclaration { typ, name, right, modifier} => {
                let mut s = String::new();
                for m in modifier {
                    s.push_str(m.as_str());
                    s.push_str(" ");
                }
                s.push_str(typ.generate().as_str());
                s.push_str(" ");
                s.push_str(name_prefix.as_str());
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
            TopLevel::Function(func) => func.generate(name_prefix),
            TopLevel::Struct(struc) => struc.generate(name_prefix),
            TopLevel::Enum(en) => en.generate(name_prefix),
            TopLevel::FunctionHeader(h) => {
                let mut s = String::new();
                s.push_str(h.generate(name_prefix).as_str());
                s.push_str(";");
                s
            },
        }
    }
}
