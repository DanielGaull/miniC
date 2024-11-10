use crate::codegen::simple::{SimpleCodeGen, IndentCodeGen};

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
    IncDec {
        identifier: IdentifierExpression,
        is_inc: bool,
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
impl IndentCodeGen for Statement {
    fn generate(&self, indent_level: usize) -> String {
        let mut s = String::new();
        let mut indent_prefix = String::new();
        for _i in 0..indent_level {
            s.push_str("    ");
            indent_prefix.push_str("    ");
        }
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
                s.push_str(self.add_body(body, indent_level + 1).as_str());
                s.push_str(indent_prefix.as_str());
                s.push_str("}");
            },
            Statement::While { condition, body } => {
                has_semicolon = false;
                s.push_str("while (");
                s.push_str(condition.generate().as_str());
                s.push_str(") {\n");
                s.push_str(self.add_body(body, indent_level + 1).as_str());
                s.push_str(indent_prefix.as_str());
                s.push_str("}");
            },
            Statement::DoWhile { condition, body } => {
                s.push_str("do {\n");
                s.push_str(&self.add_body(body, indent_level + 1).as_str());
                s.push_str(indent_prefix.as_str());
                s.push_str("} while (");
                s.push_str(condition.generate().as_str());
                s.push_str(")");
            },
            Statement::For { init, condition, increment, body } => {
                has_semicolon = false;
                s.push_str("for (");
                s.push_str(init.generate(0).as_str());
                s.push_str(condition.generate().as_str());
                s.push_str(";");
                let generated = increment.generate(0);
                let inc = generated.as_str();
                s.push_str(&inc[..inc.len() - 1]);
                s.push_str(") {\n");
                s.push_str(self.add_body(body, indent_level + 1).as_str());
                s.push_str(indent_prefix.as_str());
                s.push_str("}");
            },
            Statement::IncDec { identifier, is_inc } => {
                s.push_str(identifier.generate().as_str());
                if *is_inc {
                    s.push_str("++");
                } else {
                    s.push_str("--");
                }
            },
        }
        if has_semicolon {
            s.push(';');
        }
        s
    }
}

impl Statement {
    fn add_body(&self, body: &Vec<Statement>, indent_level: usize) -> String {
        let mut s = String::new();
        for line in body {
            s.push_str(line.generate(indent_level).as_str());
            s.push_str("\n");
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
