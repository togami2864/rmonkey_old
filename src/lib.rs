pub mod ast;
pub mod buildin;
pub mod environment;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod operator;
pub mod parser;
pub mod token;

use std::{ffi::OsStr, fs, path::Path};

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

pub fn execute(file_path: &str) -> String {
    let ext = get_file_extension(file_path).unwrap();
    if ext == "monkey" {
        let code = fs::read_to_string(file_path).unwrap();
        let mut e = Evaluator::new();
        let l = Lexer::new(code.as_str());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        match e.eval(program) {
            Ok(o) => o.to_string(),
            Err(err) => err.to_string(),
        }
    } else {
        todo!()
    }
}

fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
