use crate::codegen::simple::SimpleCodeGen;

use super::{expression::{BinOp, Expression}, types::Type};

pub enum Statement {
    Expression(Expression),
    VarDec {
        typ: Type,
        name: String,
        right: Option<Expression>,
    },
    VarAssign {
        identifier: IdentifierExpression,
        right: Expression,
    },
    BinOpVarAssign {
        identifier: IdentifierExpression,
        op: BinOp,
        right: Expression,
    },
    Return(Expression),
}

pub enum IdentifierExpression {
    Standard(String),
    Pointer(Expression),
}
impl SimpleCodeGen for IdentifierExpression {
    fn generate(&self) -> String {
        match self {
            Self::Standard(name) => name.to_string(),
            Self::Pointer(expr) => todo!(), // TODO: once Expression is done
        }
    }
}
