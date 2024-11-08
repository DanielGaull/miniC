/**
 * This is where we'll test C files that should simply output the same code
 * i.e. we won't use any of the features of miniC, just simple programs
 * that will test out a variety of types of expressions
 */
#[cfg(test)]
mod tests {
    use std::{fs::{self, File}, io::Read, path::Path};

    use mini_c::{codegen::full::CodeGenerator, parsing::parser::MyMiniCParser};

    #[test]
    fn test_return_to_c() {
        let paths = fs::read_dir("./tests/res/return_to_c_files").expect("Could not read test files");
        for entry in paths {
            let mut generator: CodeGenerator = CodeGenerator::new();

            let path = entry.unwrap().path();
            let in_path = path.join("in.c");
            let out_path = path.join("out.c");
            let out_content = read_file_to_string(&out_path);

            let program = MyMiniCParser::parse_file(String::from(in_path.to_str().unwrap()));
            if program.is_err() {
                println!("ERROR: {}", program.err().unwrap());
                assert!(false);
            } else {
                let generated = generator.code_gen(program.unwrap());
                assert_eq!(out_content, generated);
            }
        }
    }

    fn read_file_to_string(path: &Path) ->String {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        content
    }
}
