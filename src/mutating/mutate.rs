use crate::parsing::ast::{expression::Expression, function::Function, program::Program, sstruct::Struct, statement::{self, Statement}, toplevel::TopLevel};
use super::mutators::{ExpressionMutator, StatementMutator};
use anyhow::Result;

pub struct Mutator {
    expr_mutators: Vec<Box<dyn ExpressionMutator>>,
    stmt_mutators: Vec<Box<dyn StatementMutator>>,
}

impl Mutator {
    pub fn new() -> Self {
        Mutator {
            expr_mutators: Vec::new(),
            stmt_mutators: Vec::new(),
        }
    }

    pub fn mutate_program(&self, p: Program) -> Result<Program> {
        let mut toplevels = Vec::new();
        for t in p.statements {
            toplevels.push(self.mutate_toplevel(t)?);
        }
        return Ok(Program {
            statements: toplevels,
        });
    }
    fn mutate_toplevel(&self, t: TopLevel) -> Result<TopLevel> {
        match t {
            TopLevel::Module { name, body } => {
                        let mut toplevels = Vec::new();
                        for t in body {
                            toplevels.push(self.mutate_toplevel(t)?);
                        }
                        Ok(TopLevel::Module { name: name, body: toplevels })
                    },
            TopLevel::Function(func) => Ok(TopLevel::Function(self.mutate_function(func)?)),
            TopLevel::VarDeclaration { typ, name, right, modifier } => {
                let resolved_right = if let Some(exp) = right {
                    Some(self.mutate_expression(exp)?)
                } else {
                    None
                };
                Ok(TopLevel::VarDeclaration { typ: typ, name: name, right: resolved_right, modifier: modifier })
            },
            TopLevel::Import { name, is_lib } => Ok(TopLevel::Import { name: name, is_lib: is_lib }),
            TopLevel::Struct(s) => Ok(TopLevel::Struct(self.mutate_struct(s)?)),
            TopLevel::Enum(e) => Ok(TopLevel::Enum(e)),
            TopLevel::FunctionHeader(function_header) => Ok(TopLevel::FunctionHeader(function_header)),
            TopLevel::TypeDef(type_def) => Ok(TopLevel::TypeDef(type_def)),
            TopLevel::PreprocessorDirective(str) => Ok(TopLevel::PreprocessorDirective(str)),
        }
    }
    fn mutate_function(&self, func: Function) -> Result<Function> {
        Ok(Function { header: func.header, body: self.mutate_body(func.body)? })
    }
    fn mutate_struct(&self, struc: Struct) -> Result<Struct> {
        Ok(Struct {
            name: struc.name,
            members: struc.members,
            is_anonymous: struc.is_anonymous,
            is_union: struc.is_union,
        })
    }

    pub fn add_expression_mutator(&mut self, m: Box<dyn ExpressionMutator>) {
        self.expr_mutators.push(m);
    }

    pub fn add_statement_mutator(&mut self, m: Box<dyn StatementMutator>) {
        self.stmt_mutators.push(m);
    }

    pub fn mutate_expression(&self, mut expression: Expression) -> Result<Expression> {
        for m in &self.expr_mutators {
            expression = m.mutate_expression(expression)?;
        }
        return Ok(expression);
    }

    pub fn mutate_statement(&self, mut statement: Statement) -> Result<Statement> {
        for m in &self.stmt_mutators {
            statement = m.mutate_statement(statement)?;
        }
        return Ok(statement);
    }

    pub fn mutate_body(&self, body: Vec<Statement>) -> Result<Vec<Statement>> {
        let mut next_body = Vec::new();
        for s in body {
            next_body.push(self.mutate_statement(s)?);
        }
        Ok(next_body)
    }
}
