use crate::codegen::simple::{ModuleMemberCodeGen, SimpleCodeGen};

pub struct Enum {
    pub name: String,
    pub entries: Vec<EnumEntry>,
}
impl ModuleMemberCodeGen for Enum {
    fn generate(&self, name_prefix: &String) -> String {
        let mut s = String::new();
        s.push_str("typedef enum ");
        s.push_str(self.name.as_str());
        s.push_str("__enum {\n");
        for i in 0..self.entries.len() {
            s.push_str("    ");
            s.push_str(self.entries[i].generate().as_str());
            if i + 1 < self.entries.len() {
                s.push_str(",");
            }
            s.push_str("\n");
        }
        s.push_str("} ");
        s.push_str(name_prefix.as_str());
        s.push_str(self.name.as_str());
        s.push_str(";");
        s
    }
}

pub struct EnumEntry {
    pub name: String,
    pub value: Option<i32>,
}
impl SimpleCodeGen for EnumEntry {
    fn generate(&self) -> String {
        let mut s = String::new();
        s.push_str(self.name.as_str());
        if let Some(val) = self.value {
            let res = format!(" = {}", val);
            s.push_str(res.as_str());
        }
        s
    }
}
