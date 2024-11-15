use crate::codegen::simple::SimpleCodeGen;

use super::{identifier::Identifier, types::Type};

pub enum Atom {
    Char(u8),
    Short(i16),
    Int(i32),
    TrueLong(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    String(String),
    Identifier(Identifier),
    TypeCast {
        typ: Type,
        value: Box<Expression>,
    },
    UnaryOperation {
        op: UnaryOp,
        value: Box<Expression>,
    },
    SizeOf(Type),
    Wrapped(Box<Expression>),
}
impl SimpleCodeGen for Atom {
    fn generate(&self) -> String {
        match self {
            Atom::Char(c) => {
                let mut s = String::new();
                s.push_str("'");
                s.push(*c as char);
                s.push_str("'");
                s
            },
            Atom::Short(v) => {
                let mut s = String::new();
                s.push_str("(short)");
                s.push_str(v.to_string().as_str());
                s
            },
            Atom::Int(v) => v.to_string(),
            Atom::TrueLong(v) => {
                let mut s = String::new();
                s.push_str(v.to_string().as_str());
                s.push_str("L");
                s
            },
            Atom::Float(v) => {
                let mut s = String::new();
                s.push_str(v.to_string().as_str());
                s.push_str("f");
                s
            },
            Atom::Double(v) => v.to_string(),
            Atom::Boolean(v) => String::from(if *v { "1" } else { "0" }),
            Atom::String(v) => {
                let mut s = String::new();
                s.push_str("\"");
                s.push_str(v.as_str());
                s.push_str("\"");
                s
            },
            Atom::Identifier(ident) => ident.generate(),
            Atom::TypeCast { typ, value } => {
                let mut s = String::new();
                s.push_str("(");
                s.push_str(typ.generate().as_str());
                s.push_str(")");
                s.push_str(value.generate().as_str());
                s
            },
            Atom::UnaryOperation { op, value } => {
                let mut s = String::new();
                s.push_str(op.generate().as_str());
                s.push_str(value.generate().as_str());
                s
            },
            Atom::SizeOf(typ) => {
                let mut s = String::new();
                s.push_str("sizeof(");
                s.push_str(typ.generate().as_str());
                s.push_str(")");
                s
            },
            Atom::Wrapped(expr) => {
                let mut s = String::new();
                s.push_str("(");
                s.push_str(expr.generate().as_str());
                s.push_str(")");
                s
            },
        }
    }
}

pub enum ExprTail {
    None,
    Call {
        body: Vec<Expression>,
        next: Box<ExprTail>,
    },
    BinaryOp {
        op: BinOp,
        right: Box<Expression>,
        next: Box<ExprTail>,
    },
    MemberAccess {
        member: String,
        next: Box<ExprTail>,
    },
    PointerAccess {
        member: String,
        next: Box<ExprTail>,
    },
    Index {
        inner: Box<Expression>,
        next: Box<ExprTail>,
    },
    TernaryConditional {
        second: Box<Expression>,
        third: Box<Expression>,
        next: Box<ExprTail>,
    },
}
impl SimpleCodeGen for ExprTail {
    fn generate(&self) -> String {
        match self {
            ExprTail::None => String::new(),
            ExprTail::Call { body, next } => {
                let mut s = String::new();
                let mut arg_strings = Vec::<String>::new();
                for arg in body {
                    arg_strings.push(arg.generate());
                }
                s.push_str("(");
                s.push_str(arg_strings.join(", ").as_str());
                s.push_str(")");
                s.push_str(next.generate().as_str());
                s
            },
            ExprTail::BinaryOp { op, right, next } => {
                let mut s = String::new();
                s.push_str(op.generate().as_str());
                s.push_str(right.generate().as_str());
                s.push_str(next.generate().as_str());
                s
            },
            ExprTail::Index { inner, next } => {
                let mut s = String::new();
                s.push('[');
                s.push_str(inner.generate().as_str());
                s.push(']');
                s.push_str(next.generate().as_str());
                s
            },
            ExprTail::MemberAccess { member, next } => {
                let mut s = String::new();
                s.push_str(".");
                s.push_str(member.as_str());
                s.push_str(next.generate().as_str());
                s
            },
            ExprTail::PointerAccess { member, next } => {
                let mut s = String::new();
                s.push_str("->");
                s.push_str(member.as_str());
                s.push_str(next.generate().as_str());
                s
            },
            ExprTail::TernaryConditional { second, third, next } => {
                let mut s = String::new();
                s.push_str(" ? ");
                s.push_str(second.generate().as_str());
                s.push_str(" : ");
                s.push_str(third.generate().as_str());
                s.push_str(next.generate().as_str());
                s
            },
        }
    }
}

pub struct Expression {
    pub atom: Atom,
    pub tail: ExprTail,
}
impl SimpleCodeGen for Expression {
    fn generate(&self) -> String {
        let mut s = String::new();
        s.push_str(self.atom.generate().as_str());
        s.push_str(self.tail.generate().as_str());
        s
    }
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LeftShift,
    RightShift,
    BitOr,
    BitAnd,
    BitXor,
    LogicOr,
    LogicAnd,
    IsEqual,
    IsNotEqual,
    IsLT,
    IsLTE,
    IsGT,
    IsGTE,
}
impl SimpleCodeGen for BinOp {
    fn generate(&self) -> String {
        match self {
            BinOp::Add => String::from("+"),
            BinOp::Sub => String::from("-"),
            BinOp::Mul => String::from("*"),
            BinOp::Div => String::from("/"),
            BinOp::Mod => String::from("%"),
            BinOp::LeftShift => String::from("<<"),
            BinOp::RightShift => String::from(">>"),
            BinOp::BitOr => String::from("|"),
            BinOp::BitAnd => String::from("&"),
            BinOp::BitXor => String::from("^"),
            BinOp::LogicOr => String::from("||"),
            BinOp::LogicAnd => String::from("&&"),
            BinOp::IsEqual => String::from("=="),
            BinOp::IsNotEqual => String::from("!="),
            BinOp::IsLT => String::from("<"),
            BinOp::IsLTE => String::from("<="),
            BinOp::IsGT => String::from(">"),
            BinOp::IsGTE => String::from(">="),
        }
    }
}

pub enum UnaryOp {
    Plus,
    Minus,
    BitNot,
    LogicNot,
    AddressOf,
    Dereference,
    Increment,
    Decrement,
}
impl SimpleCodeGen for UnaryOp {
    fn generate(&self) -> String {
        match self {
            UnaryOp::Plus => String::from("+"),
            UnaryOp::Minus => String::from("-"),
            UnaryOp::BitNot => String::from("~"),
            UnaryOp::LogicNot => String::from("!"),
            UnaryOp::AddressOf => String::from("&"),
            UnaryOp::Dereference => String::from("*"),
            UnaryOp::Increment => String::from("++"),
            UnaryOp::Decrement => String::from("--"),
        }
    }
}
