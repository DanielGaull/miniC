use crate::parsing::ast::program::Program;

use super::simple::SimpleCodeGen;

pub struct CodeGenerator {
    // No fields for now
}

impl CodeGenerator {
    pub fn code_gen(&mut self, program: Program) -> String {
        let mut s = String::new();

        for top in program.statements {
            s.push_str(top.generate().as_str());
            s.push_str("\n");
        }

        s
    }
}
