use crate::lexer;
use crate::token;
use std::io;
use std::io::Write;

const PROMPT: &str = ">> ";

fn start() {
    loop {
        println!("{}", PROMPT);
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let mut l = lexer::Lexer::new(&input);

        while l.ch != '\x00' {
            println!("{:?}\n", l.nextToken());
        }
    }
}
