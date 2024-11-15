use crate::parsing::ast::program::Program;

use super::simple::ModuleMemberCodeGen;

pub struct CodeGenerator {
    // No fields for now
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            
        }
    }

    pub fn code_gen(&mut self, program: Program) -> String {
        let mut s = String::new();

        for top in program.statements {
            s.push_str(top.generate(&String::new()).as_str());
            s.push_str("\n");
        }

        s
    }
}
