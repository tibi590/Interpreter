use std::{self, io::{self, Write}};
use std::env;
use std::fs;

mod interpreter;
use interpreter::lexer;
use interpreter::token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        file(args[1].clone());
    } else {
        inline()
    }
}

fn file(file_path: String) {
    let contents: Vec<char> = fs::read_to_string(file_path.clone()).expect("Succesfull Read From File").chars().collect();

    let mut lexer = lexer::Lexer{ 
        text: contents.clone(), 
        len: contents.len(),
        tokens: vec![],
        pos: 0,
        ch: contents[0],
        line: 0,
        linepos: 0,
    };

    let tokens: Vec<token::Token>;

    tokens = lexer.tokenize();
    
    println!("##### TOKENS START #####");
    for token in tokens.into_iter() {
        println!("{}", token);
    }
    println!("##### TOKENS END #####");
}

fn inline() {
    loop {
        let mut input: String = String::new();

        print!("\n-> ");
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let contents: Vec<char> = input.chars().collect();

        let mut lexer = lexer::Lexer{ 
            text: contents.clone(), 
            len: contents.len(),
            tokens: vec![],
            pos: 0,
            ch: contents[0],
            line: 0,
            linepos: 0,
        };

        let tokens: Vec<token::Token>;

        tokens = lexer.tokenize();
        
        println!("##### TOKENS START #####");
        for token in tokens.into_iter() {
            println!("{}", token);
        }
        println!("##### TOKENS END #####");
    }
}
