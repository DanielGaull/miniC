use crate::codegen::simple::{SimpleCodeGen, IndentCodeGen};

use super::{expression::{Atom, BinOp, Expression}, types::Type};

pub struct ConditionBody {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

pub struct CaseStatement {
    pub atom: Atom,
    pub body: Vec<Statement>,
}

pub enum Statement {
    Expression(Expression),
    VarDec {
        typ: Type,
        name: String,
        right: Option<Expression>,
        modifier: Vec<String>,
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
    Return(Option<Expression>),
    If {
        base: ConditionBody,
        elseifs: Vec<ConditionBody>,
        tail: Option<Vec<Statement>>,
    },
    While(ConditionBody),
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
    Switch {
        atom: Atom,
        cases: Vec<CaseStatement>,
        default: Option<Vec<Statement>>,
    },
    Continue,
    Break,
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
            Statement::VarDec { typ, name, right, modifier } => {
                for m in modifier {
                    s.push_str(m.as_str());
                    s.push_str(" ");
                }

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
                if let Some(val) = expr {
                    s.push_str("return ");
                    s.push_str(val.generate().as_str());
                } else {
                    s.push_str("return");
                }
            },
            Statement::If { base, elseifs, tail } => {
                has_semicolon = false;
                s.push_str("if (");
                s.push_str(base.condition.generate().as_str());
                s.push_str(") {\n");
                s.push_str(self.add_body(&base.body, indent_level + 1).as_str());
                s.push_str(indent_prefix.as_str());
                s.push_str("}");
                for elseif in elseifs {
                    s.push_str("else if (");
                    s.push_str(elseif.condition.generate().as_str());
                    s.push_str(") {\n");
                    s.push_str(self.add_body(&elseif.body, indent_level + 1).as_str());
                    s.push_str(indent_prefix.as_str());
                    s.push_str("}");
                }
                if let Some(els) = tail {
                    s.push_str("else {\n");
                    s.push_str(self.add_body(&els, indent_level + 1).as_str());
                    s.push_str(indent_prefix.as_str());
                    s.push_str("}");
                }
            },
            Statement::While(condition_body) => {
                has_semicolon = false;
                s.push_str("while (");
                s.push_str(condition_body.condition.generate().as_str());
                s.push_str(") {\n");
                s.push_str(self.add_body(&condition_body.body, indent_level + 1).as_str());
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
            Statement::Switch { atom, cases, default } => {
                has_semicolon = false;
                s.push_str("switch (");
                s.push_str(atom.generate().as_str());
                s.push_str(") {\n");
                for case in cases {
                    s.push_str(&indent_prefix);
                    s.push_str("    ");
                    s.push_str("case ");
                    s.push_str(case.atom.generate().as_str());
                    s.push_str(":\n");
                    for statement in &case.body {
                        s.push_str(statement.generate(indent_level + 2).as_str());
                        s.push_str("\n");
                    }
                }
                if let Some(the_default) = default {
                    s.push_str(&indent_prefix);
                    s.push_str("    ");
                    s.push_str("default:\n");
                    for statement in the_default {
                        s.push_str(statement.generate(indent_level + 2).as_str());
                        s.push_str("\n");
                    }
                }
                s.push_str(&indent_prefix);
                s.push_str("}\n");
            },
            Statement::Continue => s.push_str("continue"),
            Statement::Break => s.push_str("break"),
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
