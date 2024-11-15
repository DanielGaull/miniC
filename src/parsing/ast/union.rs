use crate::codegen::simple::{ModuleMemberCodeGen, SimpleCodeGen};

use super::sstruct::StructField;

pub struct Union {
    pub name: String,
    pub fields: Vec<StructField>,
}

impl ModuleMemberCodeGen for Union {
    fn generate(&self, name_prefix: &String) -> String {
        let mut s = String::new();
        s.push_str("typedef union ");
        s.push_str(self.name.as_str());
        s.push_str("__union {\n");
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
