use crate::codegen::simple::{ModuleMemberCodeGen, PureCodeGen, SimpleCodeGen};

use super::types::Type;

pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub is_anonymous: bool,
}
impl ModuleMemberCodeGen for Struct {
    fn generate(&self, name_prefix: &String) -> String {
        if self.is_anonymous {
            // Always pure-generate anonymous structs
            return self.generate_pure();
        }

        let mut s = String::new();
        s.push_str("typedef struct ");
        s.push_str(self.name.as_str());
        s.push_str("__struct ");
        s.push_str("{\n");
        for field in &self.fields {
            s.push_str("    ");
            s.push_str(field.generate().as_str());
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
    fn generate_pure(&self) -> String {
        let mut s = String::new();
        s.push_str("struct ");
        if !self.is_anonymous {
            s.push_str(self.name.as_str());
            s.push_str(" ");
        }
        s.push_str("{\n");
        for field in &self.fields {
            s.push_str("    ");
            s.push_str(field.generate().as_str());
            s.push_str("\n");
        }
        s.push_str("}");
        s
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
