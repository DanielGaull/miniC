use crate::codegen::simple::SimpleCodeGen;

pub struct Type {
    pub is_struct: bool,
    pub name: String,
    pub is_pointer: bool
}

impl SimpleCodeGen for Type {
    fn generate(&self) -> String {
        let mut result: String = String::new();
        if self.is_struct {
            result.push_str("struct ");
        }
        result.push_str(self.name.as_str());
        if self.is_pointer {
            result.push_str("*");
        }
        result
    }
}
