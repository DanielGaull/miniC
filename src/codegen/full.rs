use crate::{mutating::mutator::Mutator, parsing::ast::{expression::Expression, program::Program, statement::Statement}};
use anyhow::Result;
use super::simple::ModuleMemberCodeGen;

pub struct CodeGenerator {
    mutator: Mutator,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            mutator: Mutator::new(),
        }
    }

    pub fn add_expression_mutator(&mut self, m: Box<fn(Expression) -> Result<Expression>>) {
        self.mutator.add_expression_mutator(m);
    }
    pub fn add_statement_mutator(&mut self, m: Box<fn(Statement) -> Result<Statement>>) {
        self.mutator.add_statement_mutator(m);
    }

    pub fn code_gen(&mut self, program: Program) -> Result<String> {
        let mut s = String::new();

        let mutated_program = self.mutator.mutate_program(program)?;
        for top in mutated_program.statements {
            s.push_str(top.generate(&String::new()).as_str());
            s.push_str("\n");
        }
        Ok(s)
    }
}
