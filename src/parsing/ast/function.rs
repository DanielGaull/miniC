use crate::codegen::simple::{SimpleCodeGen, IndentCodeGen};

use super::{statement::Statement, types::Type};

pub struct FunctionHeader {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Parameter>,
    pub is_extern: bool,
}
impl SimpleCodeGen for FunctionHeader {
    fn generate(&self) -> String {
        let mut s = String::new();
        if self.is_extern {
            s.push_str("extern ");
        }
        s.push_str(self.return_type.generate().as_str());
        s.push_str(" ");
        s.push_str(self.name.as_str());
        s.push_str("(");
        s.push_str(self.params.iter().map(|p| p.generate()).collect::<Vec<String>>().join(", ").as_str());
        s.push_str(")");
        s
    }
}

pub struct Function {
    pub header: FunctionHeader,
    pub body: Vec<Statement>,
}
impl IndentCodeGen for Function {
    fn generate(&self, indent: usize) -> String {
        let mut lines = Vec::<String>::new();

        let mut s = String::new();
        s.push_str(self.header.generate().as_str());
        s.push_str(" {");
        lines.push(s);

        for statement in &self.body {
            lines.push(statement.generate(indent + 1));
        }
        lines.push(String::from("}"));

        let mut result = String::new();
        for line in lines {
            for _i in 0..indent {
                result.push_str("    ")
            }
            result.push_str(line.as_str());
            result.push_str("\n");
        }
        result
    }
}

pub struct Parameter {
    pub name: String,
    pub typ: Type,
}
impl SimpleCodeGen for Parameter {
    fn generate(&self) -> String {
        let mut s = String::new();
        s.push_str(self.typ.generate().as_str());
        s.push_str(" ");
        s.push_str(self.name.as_str());
        s
    }
}
