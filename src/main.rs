pub mod lexer;

use std::{io::{self, Write}, vec};

fn main() {
    loop {
        let mut input: String = String::new();

        print!("-> ");
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let lexer = lexer::Lexer{ text: input.clone(), pos: 0, char: input.clone().chars().nth(0).unwrap(), tokens: vec![] };

        let tokens: Vec<lexer::Token> = lexer.tokenize();

        for token in tokens.into_iter() {
            println!("{}", token);
        }
    }
}
