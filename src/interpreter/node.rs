use std::fmt;

use crate::interpreter::token::*;

pub enum Expr {
    Equality {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    },
    Comparision {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    },
    Term {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    },
    Factor {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    },
    Unary {
        op: Token,
        right: Box<Expr>
    },
    Literal(Box<Literals>)
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Equality { left, op, right } => {
                write!(f, "({} {} {})", left, op.value, right)
            },
            Expr::Comparision { left, op, right } => {
                write!(f, "({} {} {})", left, op.value, right)
            },
            Expr::Term { left, op, right } => {
                write!(f, "({} {} {})", left, op.value, right)
            },
            Expr::Factor { left, op, right } => {
                write!(f, "({} {} {})", left, op.value, right)
            },
            Expr::Unary { op, right } => {
                write!(f, "({} {})", op.value, right)
            },
            Expr::Literal(value) => {
                write!(f, "({})", value)
            }
        }
    }
}

pub enum Literals {
    Int(f64),
    String(String),
    True,
    False,
    Nil,
    Expr(Expr)
}


impl std::fmt::Display for Literals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literals::Int(value) => write!(f, "{}", value),
            Literals::String(value) => write!(f, "{}", value),
            Literals::True => write!(f, "True"),
            Literals::False => write!(f, "False"),
            Literals::Nil => write!(f, "Nil"),
            Literals::Expr(expr) => write!(f, "{}", expr),
        }
    }
}
