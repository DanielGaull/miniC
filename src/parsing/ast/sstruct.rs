use crate::codegen::simple::SimpleCodeGen;

use super::types::Type;

pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}
impl SimpleCodeGen for Struct {
    fn generate(&self) -> String {
        let mut s = String::new();
        s.push_str("typedef struct ");
        s.push_str(self.name.as_str());
        s.push_str("__struct {\n");
        for field in &self.fields {
            s.push_str("    ");
            s.push_str(field.generate().as_str());
            s.push_str("\n");
        }
        s.push_str("} ");
        s.push_str(self.name.as_str());
        s.push_str(";");
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
