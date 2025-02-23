use crate::parsing::ast::{expression::Expression, statement::Statement};
use anyhow::Result;

pub trait ExpressionMutator {
    fn mutate_expression(&self, expr: Expression) -> Result<Expression>;
}

pub trait StatementMutator {
    fn mutate_statement(&self, stmt: Statement) -> Result<Statement>;
}
