use std::{self, io::{self, Write}};
use std::env;
use std::fs;

mod interpreter;
use interpreter::lexer;
use interpreter::error;
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
    let contents = fs::read_to_string(file_path.clone()).expect("Succesfull Read From File");
    let mut lines: Vec<String> = vec![];

    for line in contents.split('\n') {
        lines.push(line.trim_end().to_string());
    }

    let lexer = lexer::Lexer{ 
        file_location: file_path,
        text: contents.clone(), 
        line_text: lines,
        char: contents.clone().chars().nth(0).unwrap(),
        ..Default::default()
    };

    let tokens: Vec<token::Token>;
    let errors: Vec<error::Error>;

    (errors, tokens) = lexer.tokenize();
    
    println!("##### TOKENS START #####");
    for token in tokens.into_iter() {
        println!("{}", token);
    }
    println!("##### TOKENS END #####");
    println!("---------------------------------");
    println!("##### ERRORS START #####");
    for error in errors.into_iter() {
        println!("{}", error);
    }
    println!("##### ERRORS END #####");
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

        let lexer = lexer::Lexer{ 
            text: input.clone(), 
            line_text: vec![input.clone().trim().to_string()],
            char: input.clone().chars().nth(0).unwrap(),
            ..Default::default()
        };

        let tokens: Vec<token::Token>;
        let errors: Vec<error::Error>;

        (errors, tokens) = lexer.tokenize();
        
        println!("##### TOKENS START #####");
        for token in tokens.into_iter() {
            println!("{}", token);
        }
        println!("##### TOKENS END #####");
        println!("---------------------------------");
        println!("##### ERRORS START #####");
        for error in errors.into_iter() {
            println!("{}", error);
        }
        println!("##### ERRORS END #####");
    }
}
