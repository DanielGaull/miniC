use crate::codegen::simple::{PureCodeGen, ModuleMemberCodeGen, SimpleCodeGen};

use super::sstruct::StructField;

pub struct Union {
    pub name: String,
    pub fields: Vec<StructField>,
    pub is_anonymous: bool,
}

impl ModuleMemberCodeGen for Union {
    fn generate(&self, name_prefix: &String) -> String {
        if self.is_anonymous {
            // Always pure-generate anonymous unions
            return self.generate_pure();
        }

        let mut s = String::new();
        s.push_str("typedef union ");
        s.push_str(self.name.as_str());
        s.push_str("__union ");
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

impl PureCodeGen for Union {
    fn generate_pure(&self) -> String {
        let mut s = String::new();
        s.push_str("union ");
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
