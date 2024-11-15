use crate::codegen::simple::SimpleCodeGen;

pub enum Identifier {
    Plain(String),
    Module(String, String),
}

impl SimpleCodeGen for Identifier {
    fn generate(&self) -> String {
        match self {
            Self::Plain(name) => name.clone(),
            Self::Module(parent, child) => format!("mod__{}__{}", parent, child),
        }
    }
}
