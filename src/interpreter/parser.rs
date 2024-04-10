use crate::interpreter::token::*;
use crate::interpreter::node::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize
}

impl Parser {
    fn advance(&mut self) {
        if self.is_at_end() {
            self.pos += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        return self.tokens[self.pos].t_type != TType::EOF;
    }

    fn current(&self) -> Token {
        return self.tokens[self.pos].clone();
    }

    fn next(&self) -> Token {
        if self.is_at_end() {
            return self.tokens[self.pos+1].clone();
        }

        return Token {
            value: "<EOF>".to_string(),
            t_type: TType::EOF,
            position: 0,
            line: 0
        };
    }
    
    fn previous(&self) -> Token {
        return self.tokens[self.pos-1].clone();
    }

    pub fn parse(&mut self) -> Expr {
        let ast: Expr = self.expression();

        return ast;
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while match self.next().t_type {
            TType::BangEqual | TType::EqualEqual => {
                self.advance();
                self.advance();
                true
            },
            _ => {
                false
            }
        } {
            let op: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Equality {
                left: Box::new(expr),
                op: op,
                right: Box::new(right)
            };
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while match self.next().t_type {
            TType::Less | 
            TType::LessEqual |
            TType::Greater |
            TType::GreaterEqual => {
                self.advance();
                self.advance();
                true
            },
            _ => {
                false
            }
        } {
            let op: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Comparision {
                left: Box::new(expr),
                op: op,
                right: Box::new(right)
            };
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while match self.next().t_type {
            TType::Plus |
            TType::Minus => {
                self.advance();
                self.advance();
                true
            },
            _ => {
                false
            }
        } {
            let op: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Term {
                left: Box::new(expr),
                op: op,
                right: Box::new(right)
            };
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while match self.next().t_type {
            TType::Multi |
            TType::Div => {
                self.advance();
                self.advance();
                true
            },
            _ => {
                false
            }
        } {
            let op: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Factor {
                left: Box::new(expr),
                op: op,
                right: Box::new(right)
            };
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if match self.current().t_type {
            TType::Bang |
            TType::Minus => {
                self.advance();
                true
            },
            _ => {
                false
            }
        } {
            let op: Token = self.previous();
            let right: Expr = self.unary();
            let expr: Expr = Expr::Unary {
                op: op,
                right: Box::new(right)
            };
            self.advance();

            return expr;
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        let current: Token = self.current();

        match current.t_type {
            TType::Int => return Expr::Literal(
                Box::new(Literals::Int(current.value.parse::<f64>().unwrap()))
            ),
            TType::Ident => return Expr::Literal(
                Box::new(Literals::String(current.value))
            ),
            TType::True => return Expr::Literal(
                Box::new(Literals::True)
            ),
            TType::False => return Expr::Literal(
                Box::new(Literals::False)
            ),
            TType::Nil => return Expr::Literal(
                Box::new(Literals::Nil)
            ),
            TType::ParenLeft => {
                self.advance();
                let expr: Expr = self.expression();

                println!("{}", self.next().t_type);
                if self.next().t_type == TType::ParenRight {
                    return expr;
                }

                return Expr::Literal(
                    Box::new(Literals::Nil)
                );
            },
            _ => return Expr::Literal(
                Box::new(Literals::Nil)
            )
        }
    }
}
