use std::io::Result;

use rmonkey::{evaluator::eval, lexer::Lexer, parser::Parser};

fn prompt(s: &str) -> Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(s.as_bytes()).unwrap();
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let l = Lexer::new(line.as_str());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match eval(program) {
                Ok(o) => {
                    println!("{}", o);
                }
                Err(_) => todo!(),
            }
        }
    }
}
