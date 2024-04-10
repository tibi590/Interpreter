use std::{self, io::{self, Write}};
use std::env;
use std::fs;

mod interpreter;
use interpreter::lexer;
use interpreter::token;
use interpreter::node;

use crate::interpreter::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        file(args[1].clone());
    } else {
        inline()
    }
}

fn file(file_path: String) {
    let contents: Vec<u8> = fs::read(file_path.clone())
        .expect("Error reading from file");

    let lexer = lexer::Lexer{ 
        text: contents.clone(), 
        len: contents.len(),
        tokens: vec![],
        pos: 0,
        ch: contents[0],
        line: 0,
        linepos: 0,
    };

    let tokens: Vec<token::Token> = lexer.tokenize();
    
    println!("##### TOKENS START #####");
    for token in tokens.clone().into_iter() {
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

        let contents: Vec<u8> = input.as_bytes().to_vec();

        let lexer = lexer::Lexer{ 
            text: contents.clone(), 
            len: contents.len(),
            tokens: vec![],
            pos: 0,
            ch: contents[0],
            line: 0,
            linepos: 0,
        };

        let tokens: Vec<token::Token> = lexer.tokenize();
        
        println!("##### TOKENS START #####");
        for token in tokens.clone().into_iter() {
            println!("{}", token);
        }
        println!("##### TOKENS END #####");

        let mut parser: Parser = Parser {
            tokens: tokens,
            pos: 0
        };

        println!("\n##### AST START #####");
        let ast = parser.parse();

        println!("{}", tree_walker(ast));

        println!("##### AST END #####");
    }
}

fn tree_walker(node: node::Expr) -> String {
    match node {
        node::Expr::Equality { left, op, right } => {
            return format!("({} {} {})", tree_walker(*left), op.value, tree_walker(*right));
        },

        node::Expr::Comparision { left, op, right } => {
            return format!("({} {} {})", tree_walker(*left), op.value, tree_walker(*right));
        },

        node::Expr::Term { left, op, right } => {
            return format!("({} {} {})", tree_walker(*left), op.value, tree_walker(*right));
        },
        
        node::Expr::Factor { left, op, right } => {
            return format!("({} {} {})", tree_walker(*left), op.value, tree_walker(*right));
        },

        node::Expr::Unary { op, right } => {
            return format!("({} {})", op.value, tree_walker(*right));
        },

        node::Expr::Literal(value) => {
            return format!("({value})");
        }
    }
}
