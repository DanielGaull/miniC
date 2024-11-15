pub trait SimpleCodeGen {
    fn generate(&self) -> String;
}

pub trait IndentCodeGen {
    fn generate(&self, indent_level: usize) -> String;
}

pub trait ModuleMemberCodeGen {
    fn generate(&self, name_prefix: &String) -> String;
}

pub trait PureCodeGen {
    fn generate_pure(&self) -> String;
}
