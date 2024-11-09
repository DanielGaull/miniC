pub trait SimpleCodeGen {
    fn generate(&self) -> String;
}

pub trait IndentCodeGen {
    fn generate(&self, indent_level: usize) -> String;
}
