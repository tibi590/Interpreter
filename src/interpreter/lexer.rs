use crate::interpreter::token::{Token, TType};

pub struct Lexer {
    pub text: Vec<char>, 
    pub len: usize,
    pub tokens: Vec<Token>,
    pub pos: usize,
    pub ch: char,
    pub line: usize,
    pub linepos: usize
}

impl Lexer {
    fn advance(&mut self) {
        self.pos += 1;

        if self.not_at_end(self.pos) {
            self.ch = self.text[self.pos];
            self.linepos += 1;
        }
    }

    fn get_next(&self) -> char {
        return self.text[self.pos+1];
    }

    fn not_at_end(&self, pos: usize) -> bool {
        return pos < self.len;
    }

    fn make_num(&mut self) -> Token {
        let mut value: String = String::new();
        let mut is_numeric: bool = true;

        while is_numeric {
            value.push(self.ch);
            if self.get_next().is_numeric() {
                self.advance();
            } else {
                is_numeric = false;
            }
        }

        return Token { 
            value,
            t_type: TType::Int, 
            position: self.linepos, 
            line: self.line 
        };
    }

    pub fn tokenize(mut self) -> Vec<Token>{
        while self.pos != self.len {
            match self.ch {
                ' ' => { },
                '\n' => { 
                    self.line += 1; 
                    self.linepos = 0;
                },

                '(' => { self.tokens.push(Token { 
                    value: String::from('('), 
                    t_type: TType::ParenLeft, 
                    position: self.linepos, 
                    line: self.line }); 
                },

                ')' => { self.tokens.push(Token { 
                    value: String::from(')'), 
                    t_type: TType::ParenRight, 
                    position: self.linepos, 
                    line: self.line }); 
                },
            
                '+' => { self.tokens.push(Token { 
                    value: String::from('+'), 
                    t_type: TType::Plus, 
                    position: self.linepos, 
                    line: self.line }); 
                },
                
                '-' => { self.tokens.push(Token { 
                    value: String::from('-'),
                    t_type: TType::Minus,
                    position: self.linepos,
                    line: self.line }); 
                },

                '*' => { self.tokens.push(Token { 
                    value: String::from('*'),
                    t_type: TType::Multi,
                    position: self.linepos,
                    line: self.line });
                },

                '/' => { self.tokens.push(Token { 
                    value: String::from('/'),
                    t_type: TType::Div,
                    position: self.linepos,
                    line: self.line });
                },

                '0' ..= '9' => { 
                    let token: Token = self.make_num();
                    self.tokens.push(token); 
                },
                _ => { println!("ERROR: Syntax Error") }
            }

            self.advance();
        }

        self.tokens.push(Token { 
            value: String::from("<EOF>"), 
            t_type: TType::EOF,
            position: self.linepos,
            line: self.line 
        });

        return self.tokens;
    }
}
