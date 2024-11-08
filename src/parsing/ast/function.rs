use crate::codegen::simple::SimpleCodeGen;

use super::{statement::Statement, types::Type};

pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
}
impl SimpleCodeGen for Function {
    fn generate(&self) -> String {
        let mut s = String::new();
        s.push_str(self.return_type.generate().as_str());
        s.push_str(" ");
        s.push_str(self.name.as_str());
        s.push_str("(");
        s.push_str(self.params.iter().map(|p| p.generate()).collect::<Vec<String>>().join(", ").as_str());
        s.push_str(") {\n");
        for statement in &self.body {
            s.push_str("\t");
            s.push_str(statement.generate().as_str());
            s.push_str("\n");
        }
        s.push_str("}\n");
        s
    }
}

pub struct Parameter {
    pub name: String,
    pub typ: Type,
}
impl SimpleCodeGen for Parameter {
    fn generate(&self) -> String {
        todo!()
    }
}
