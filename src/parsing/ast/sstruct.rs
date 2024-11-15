use crate::codegen::simple::{IndentCodeGen, ModuleMemberCodeGen, PureCodeGen, SimpleCodeGen};

use super::types::Type;

pub struct Struct {
    pub name: String,
    pub members: Vec<StructMember>,
    pub is_anonymous: bool,
    pub is_union: bool,
}
impl ModuleMemberCodeGen for Struct {
    fn generate(&self, name_prefix: &String) -> String {
        if self.is_anonymous {
            // Always pure-generate anonymous structs
            return self.generate_pure(0);
        }

        let mut s = String::new();
        let struct_name = if self.is_union { "union" } else { "struct" };

        s.push_str("typedef ");
        s.push_str(struct_name);
        s.push_str(" ");
        s.push_str(self.name.as_str());
        s.push_str("__");
        s.push_str(struct_name);
        s.push_str(" ");
        s.push_str("{\n");
        for member in &self.members {
            s.push_str(member.generate(1).as_str());
            s.push_str("\n");
        }
        s.push_str("} ");
        s.push_str(name_prefix.as_str());
        s.push_str(self.name.as_str());
        s.push_str(";");
        s
    }
}

impl PureCodeGen for Struct {
    fn generate_pure(&self, indent_level: usize) -> String {
        let mut s = String::new();
        let prefix = "    ".repeat(indent_level);
        let struct_name = if self.is_union { "union" } else { "struct" };

        s.push_str(prefix.as_str());
        s.push_str(struct_name);
        s.push_str(" ");
        if !self.is_anonymous {
            s.push_str(self.name.as_str());
            s.push_str(" ");
        }
        s.push_str("{\n");
        for member in &self.members {
            s.push_str(member.generate(indent_level + 1).as_str());
            s.push_str("\n");
        }
        s.push_str(prefix.as_str());
        s.push_str("}");
        s
    }
}

pub enum StructMember {
    Field(StructField),
    AnonStruct(Struct),
}
impl IndentCodeGen for StructMember {
    fn generate(&self, indent: usize) -> String {
        match self {
            Self::Field(f) => {
                let mut str = String::new();
                str.push_str("    ".repeat(indent).as_str());
                str.push_str(f.generate().as_str());
                str
            },
            Self::AnonStruct(s) => {
                let mut str = String::new();
                str.push_str(s.generate_pure(indent).as_str());
                str.push_str(";");
                str
            },
        }
    }
}

pub struct StructField {
    pub typ: Type,
    pub name: String,
}
impl SimpleCodeGen for StructField {
    fn generate(&self) -> String {
        let mut s = String::new();
        s.push_str(self.typ.generate().as_str());
        s.push_str(" ");
        s.push_str(self.name.as_str());
        s.push_str(";");
        s
    }
}
