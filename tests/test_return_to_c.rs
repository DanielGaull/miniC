/**
 * This is where we'll test C files that should simply output the same code
 * i.e. we won't use any of the features of miniC, just simple programs
 * that will test out a variety of types of expressions
 */
#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_return_to_c() {
        let paths = fs::read_dir("./res/return_to_c_files").expect("Could not read test files");
        for entry in paths {
            let entry = entry.unwrap();

        }
    }
}
