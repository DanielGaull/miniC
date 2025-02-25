/**
 * This is where we'll test C files that should simply output the same code
 * i.e. we won't use any of the features of miniC, just simple programs
 * that will test out a variety of types of expressions
 */
#[cfg(test)]
mod tests {
    use std::{fs::{self, File}, io::Read, path::Path};
    use anyhow::Result;
    use mini_c::{codegen::full::CodeGenerator, parsing::parser::MyMiniCParser};

    #[test]
    fn test_return_to_c() -> Result<()> {
        let paths = fs::read_dir("./tests/res/conversion_files").expect("Could not read test files");
        for entry in paths {
            let mut generator: CodeGenerator = CodeGenerator::new();

            let path = entry.unwrap().path();
            let in_path = path.join("in.c");
            let out_path = path.join("out.c");
            let out_content = read_file_to_string(&out_path).replace("\r\n", "\n");

            println!("Trying: {}", path.to_str().unwrap());
            let program = MyMiniCParser::parse_file(String::from(in_path.to_str().unwrap()));
            if program.is_err() {
                panic!("ERROR: {}", program.err().unwrap());
            } else {
                let generated = generator.code_gen(program.unwrap())?;
                assert_eq!(out_content, generated);
            }
        }
        Ok(())
    }

    #[test]
    fn test_mutations() -> Result<()> {
        let paths = fs::read_dir("./tests/res/mutate_files").expect("Could not read test files");
        for entry in paths {
            let mut generator: CodeGenerator = CodeGenerator::new();

            let path = entry.unwrap().path();
            let in_path = path.join("in.c");
            let out_path = path.join("out.c");
            let out_content = read_file_to_string(&out_path).replace("\r\n", "\n");

            println!("Trying: {}", path.to_str().unwrap());
            let program = MyMiniCParser::parse_file(String::from(in_path.to_str().unwrap()));
            if program.is_err() {
                panic!("ERROR: {}", program.err().unwrap());
            } else {
                let generated = generator.code_gen(program.unwrap())?;
                assert_eq!(out_content, generated);
            }
        }
        Ok(())
    }

    fn read_file_to_string(path: &Path) ->String {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        content
    }
}
