use super::{expression::Expression, types::Type};

pub enum TopLevel {
    VarDeclaration {
        typ: Type,
        name: String,
        right: Option<Expression>,
    },
    Import(String),
}
