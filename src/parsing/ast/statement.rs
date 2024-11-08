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
impl SimpleCodeGen for Statement {
    fn generate(&self) -> String {
        let mut s = String::new();
        match self {
            Statement::Expression(expr) => s.push_str(expr.generate().as_str()),
            Statement::VarDec { typ, name, right } => {
                s.push_str(typ.generate().as_str());
                s.push(' ');
                s.push_str(name.as_str());
                if let Some(body) = right {
                    s.push_str(" = ");
                    s.push_str(body.generate().as_str());
                }
            },
            Statement::VarAssign { identifier, right } => {
                s.push_str(identifier.generate().as_str());
                s.push_str(" = ");
                s.push_str(right.generate().as_str());
            },
            Statement::BinOpVarAssign { identifier, op, right } => {
                s.push_str(identifier.generate().as_str());
                s.push_str(" ");
                s.push_str(op.generate().as_str());
                s.push_str("= ");
                s.push_str(right.generate().as_str());
            },
            Statement::Return(expr) => {
                s.push_str("return ");
                s.push_str(expr.generate().as_str());
            }
        }
        s.push(';');
        s
    }
}

pub enum IdentifierExpression {
    Standard(String),
    Pointer(Expression),
}
impl SimpleCodeGen for IdentifierExpression {
    fn generate(&self) -> String {
        match self {
            Self::Standard(name) => name.to_string(),
            Self::Pointer(expr) => expr.generate(),
        }
    }
}
