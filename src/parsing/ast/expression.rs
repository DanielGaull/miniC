use crate::codegen::simple::SimpleCodeGen;

pub enum Atom {
    Char(u8),
    Short(i16),
    Int(i32),
    TrueLong(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    String(String),
    Reference(String),
    Identifier(String),
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
            Atom::Reference(ident) => {
                let mut s = String::new();
                s.push_str("&");
                s.push_str(ident.as_str());
                s
            },
            Atom::Identifier(ident) => ident.clone(),
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

// TODO: Power operator? (^)
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
