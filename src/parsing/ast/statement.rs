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
    If {
        condition: Expression,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    DoWhile {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        init: Box<Statement>,
        condition: Expression,
        increment: Box<Statement>,
        body: Vec<Statement>,
    },
}
impl SimpleCodeGen for Statement {
    fn generate(&self) -> String {
        let mut s = String::new();
        let mut has_semicolon = true;
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
            },
            Statement::If { condition, body } => {
                has_semicolon = false;
                s.push_str("if (");
                s.push_str(condition.generate().as_str());
                s.push_str(") {\n");
                for line in body {
                    s.push_str("    ");
                    s.push_str(line.generate().as_str());
                    s.push_str("\n");
                }
                s.push_str("}\n");
            },
            Statement::While { condition, body } => {
                has_semicolon = false;
                s.push_str("while (");
                s.push_str(condition.generate().as_str());
                s.push_str(") {\n");
                for line in body {
                    s.push_str("    ");
                    s.push_str(line.generate().as_str());
                    s.push_str("\n");
                }
                s.push_str("}\n");
            },
            Statement::DoWhile { condition, body } => {
                s.push_str("do {\n");
                for line in body {
                    s.push_str("    ");
                    s.push_str(line.generate().as_str());
                    s.push_str("\n");
                }
                s.push_str("} while (");
                s.push_str(condition.generate().as_str());
                s.push_str(")");
            },
            Statement::For { init, condition, increment, body } => {
                s.push_str("for (");
                s.push_str(init.generate().as_str());
                s.push_str(condition.generate().as_str());
                s.push_str(";");
                let generated = increment.generate();
                let inc = generated.as_str();
                s.push_str(&inc[..inc.len() - 1]);
                s.push_str(") {\n");
                for line in body {
                    s.push_str("    ");
                    s.push_str(line.generate().as_str());
                    s.push_str("\n");
                }
                s.push_str("}\n");
            }
        }
        if has_semicolon {
            s.push(';');
        }
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
