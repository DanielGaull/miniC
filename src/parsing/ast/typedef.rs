use crate::codegen::simple::{ModuleMemberCodeGen, PureCodeGen, SimpleCodeGen};

use super::{enumm::Enum, sstruct::Struct, types::Type, union::Union};

pub struct TypeDef {
    pub name: String,
    pub typ: TypeDefInner,
}
impl ModuleMemberCodeGen for TypeDef {
    fn generate(&self, name_prefix: &String) -> String {
        let mut s = String::new();
        s.push_str("typedef ");
        s.push_str(
            match &self.typ {
                TypeDefInner::Type(t) => t.generate(),
                TypeDefInner::Enum(e) => e.generate_pure(),
                TypeDefInner::Struct(s) => s.generate_pure(),
                TypeDefInner::Union(u) => u.generate_pure(),
            }.as_str()
        );
        s.push_str(" ");
        s.push_str(name_prefix);
        s.push_str(self.name.as_str());
        s.push_str(";");
        s
    }
}

pub enum TypeDefInner {
    Type(Type),
    Struct(Struct),
    Enum(Enum),
    Union(Union),
}
