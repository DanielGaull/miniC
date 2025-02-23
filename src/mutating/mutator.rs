use crate::parsing::ast::{expression::{Atom, ExprTail, Expression}, function::Function, program::Program, sstruct::Struct, statement::{CaseStatement, ConditionBody, Statement}, toplevel::TopLevel};
use anyhow::Result;

pub struct Mutator {
    expr_mutators: Vec<Box<fn(Expression) -> Result<Expression>>>,
    stmt_mutators: Vec<Box<fn(Statement) -> Result<Statement>>>,
}

impl Mutator {
    pub fn new() -> Self {
        Mutator {
            expr_mutators: Vec::new(),
            stmt_mutators: Vec::new(),
        }
    }

    pub fn add_expression_mutator(&mut self, m: Box<fn(Expression) -> Result<Expression>>) {
        self.expr_mutators.push(m);
    }

    pub fn add_statement_mutator(&mut self, m: Box<fn(Statement) -> Result<Statement>>) {
        self.stmt_mutators.push(m);
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

    pub fn mutate_expression(&self, mut expression: Expression) -> Result<Expression> {
        for m in &self.expr_mutators {
            expression = m(expression)?;
        }

        let atom = expression.atom;
        let tail = expression.tail;
        let mutated_atom = self.mutate_atom(atom)?;
        let mutated_tail = self.mutate_tail(tail)?;

        return Ok(Expression { atom: mutated_atom, tail: mutated_tail });
    }
    fn mutate_atom(&self, atom: Atom) -> Result<Atom> {
        match atom {
            Atom::Char(v) => Ok(Atom::Char(v)),
            Atom::Short(v) => Ok(Atom::Short(v)),
            Atom::Int(v) => Ok(Atom::Int(v)),
            Atom::TrueLong(v) => Ok(Atom::TrueLong(v)),
            Atom::Float(v) => Ok(Atom::Float(v)),
            Atom::Double(v) => Ok(Atom::Double(v)),
            Atom::Boolean(v) => Ok(Atom::Boolean(v)),
            Atom::String(v) => Ok(Atom::String(v)),
            Atom::Identifier(identifier) => Ok(Atom::Identifier(identifier)),
            Atom::TypeCast { typ, value } => {
                Ok(Atom::TypeCast { typ: typ, value: Box::new(self.mutate_expression(*value)?) })
            },
            Atom::UnaryOperation { op, value } => {
                Ok(Atom::UnaryOperation { op: op, value: Box::new(self.mutate_expression(*value)?) })
            },
            Atom::SizeOf(v) => Ok(Atom::SizeOf(v)),
            Atom::Wrapped(expression) => {
                Ok(Atom::Wrapped(Box::new(self.mutate_expression(*expression)?)))
            },
        }
    }
    fn mutate_tail(&self, tail: ExprTail) -> Result<ExprTail> {
        match tail {
            ExprTail::None => Ok(ExprTail::None),
            ExprTail::Call { body, next } => {
                Ok(
                    ExprTail::Call {
                        body: body.into_iter().map(|c| self.mutate_expression(c)).collect::<Result<_, _>>()?,
                        next: Box::new(self.mutate_tail(*next)?)
                    }
                )
            },
            ExprTail::BinaryOp { op, right, next } => {
                Ok(
                    ExprTail::BinaryOp { 
                        op: op, 
                        right: Box::new(self.mutate_expression(*right)?), 
                        next: Box::new(self.mutate_tail(*next)?) 
                    }
                )
            },
            ExprTail::MemberAccess { member, next } => {
                Ok(
                    ExprTail::MemberAccess { 
                        member: member, 
                        next: Box::new(self.mutate_tail(*next)?) 
                    }
                )
            },
            ExprTail::PointerAccess { member, next } => {
                Ok(
                    ExprTail::PointerAccess { 
                        member: member, 
                        next: Box::new(self.mutate_tail(*next)?) 
                    }
                )
            },
            ExprTail::Index { inner, next } => {
                Ok(
                    ExprTail::Index { 
                        inner: Box::new(self.mutate_expression(*inner)?), 
                        next: Box::new(self.mutate_tail(*next)?) 
                    }
                )
            },
            ExprTail::TernaryConditional { second, third, next } => {
                Ok(
                    ExprTail::TernaryConditional { 
                        second: Box::new(self.mutate_expression(*second)?), 
                        third: Box::new(self.mutate_expression(*third)?), 
                        next: Box::new(self.mutate_tail(*next)?) 
                    }
                )
            },
        }
    }

    pub fn mutate_statement(&self, mut statement: Statement) -> Result<Statement> {
        for m in &self.stmt_mutators {
            statement = m(statement)?;
        }

        match statement {
            Statement::Expression(expression) => Ok(Statement::Expression(self.mutate_expression(expression)?)),
            Statement::VarDec { typ, name, right, modifier } => {
                let resolved_right = if let Some(exp) = right {
                    Some(self.mutate_expression(exp)?)
                } else {
                    None
                };
                Ok(Statement::VarDec { typ: typ, name: name, right: resolved_right, modifier: modifier })
            },
            Statement::VarAssign { identifier, right } => {
                Ok(Statement::VarAssign { identifier: identifier, right: self.mutate_expression(right)? })
            },
            Statement::BinOpVarAssign { identifier, op, right } => {
                Ok(Statement::BinOpVarAssign { identifier: identifier, op: op, right: self.mutate_expression(right)? })
            },
            Statement::IncDec { identifier, is_inc } => Ok(Statement::IncDec { identifier: identifier, is_inc: is_inc }),
            Statement::Return(expression) => {
                Ok(Statement::Return(
                    if let Some(body) = expression {
                        Some(self.mutate_expression(body)?)
                    } else {
                        None
                    }
                ))
            },
            Statement::If { base, elseifs, tail } => {
                Ok(Statement::If {
                    base: self.mutate_condition_body(base)?,
                    elseifs: elseifs.into_iter().map(|c| self.mutate_condition_body(c)).collect::<Result<_, _>>()?,
                    tail: tail.map(|f| self.mutate_body(f)).transpose()?,
                })
            },
            Statement::While(condition_body) => {
                Ok(Statement::While(self.mutate_condition_body(condition_body)?))
            },
            Statement::DoWhile { condition, body } => {
                Ok(Statement::DoWhile { condition: self.mutate_expression(condition)?, body: self.mutate_body(body)? })
            },
            Statement::For { init, condition, increment, body } => {
                Ok(
                    Statement::For {
                        init: Box::new(self.mutate_statement(*init)?),
                        condition: self.mutate_expression(condition)?,
                        increment: Box::new(self.mutate_statement(*increment)?),
                        body: self.mutate_body(body)?,
                    }
                )
            },
            Statement::Switch { atom, cases, default } => {
                Ok(
                    Statement::Switch {
                        atom: self.mutate_atom(atom)?,
                        cases: cases.into_iter().map(|c| self.mutate_case_body(c)).collect::<Result<_, _>>()?,
                        default: default.map(|d| self.mutate_body(d)).transpose()?,
                    }
                )
            },
            Statement::Continue => Ok(Statement::Continue),
            Statement::Break => Ok(Statement::Break),
        }
    }

    pub fn mutate_body(&self, body: Vec<Statement>) -> Result<Vec<Statement>> {
        let mut next_body = Vec::new();
        for s in body {
            next_body.push(self.mutate_statement(s)?);
        }
        Ok(next_body)
    }
    fn mutate_condition_body(&self, body: ConditionBody) -> Result<ConditionBody> {
        let exp = self.mutate_expression(body.condition)?;
        let new_body = self.mutate_body(body.body)?;
        Ok(ConditionBody {
            condition: exp,
            body: new_body,
        })
    }
    fn mutate_case_body(&self, case: CaseStatement) -> Result<CaseStatement> {
        let atom = self.mutate_atom(case.atom)?;
        let body = self.mutate_body(case.body)?;
        Ok(CaseStatement { atom: atom, body: body })
    }
}
