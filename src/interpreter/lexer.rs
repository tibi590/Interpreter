use crate::interpreter::token::{Token, TType};

pub struct Lexer {
    pub text: Vec<u8>, 
    pub len: usize,
    pub tokens: Vec<Token>,
    pub pos: usize,
    pub ch: u8,
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

    fn get_next(&self) -> u8 {
        return self.text[self.pos+1];
    }

    fn not_at_end(&self, pos: usize) -> bool {
        return pos < self.len;
    }

    fn make_num(&mut self) -> Token {
        let start = self.pos;
        let mut end = self.pos;

        while (b'0'..=b'9').contains(&self.get_next()) {
            end += 1;
            if self.not_at_end(self.pos) {
                self.advance();
            }
        }

        return Token { 
            value: String::from_utf8(self.text[start..=end].to_vec())
                .expect("Error converint u8 to string"),
            t_type: TType::Int, 
            position: self.linepos, 
            line: self.line 
        };
    }

    fn make_ident(&mut self) -> Token {
        let start = self.pos;
        let mut end = self.pos;

        let mut is_valid: bool = true;
        let mut next_ch = self.get_next();

        while is_valid {
            if self.not_at_end(self.pos) && next_ch.is_ascii_alphabetic() {
                end += 1;
                self.advance();
                next_ch = self.get_next();
            } else {
                is_valid = false;
            }
        }

        let value = std::str::from_utf8(&self.text[start..=end]).expect("Error converint u8 to &str");

        match value {
            "true" => {
                return Token {
                    value: String::from(value),
                    t_type: TType::True,
                    position: self.linepos,
                    line: self.line
                }
            },

            "false" => {
                return Token {
                    value: String::from(value),
                    t_type: TType::False,
                    position: self.linepos,
                    line: self.line
                }
            },

            "if" => {
                return Token {
                    value: String::from(value),
                    t_type: TType::If,
                    position: self.linepos,
                    line: self.line
                }
            },

            _ => {
                return Token {
                    value: String::from(value),
                    t_type: TType::Ident,
                    position: self.linepos,
                    line: self.line
                }
            }
        };
    }

    pub fn tokenize(mut self) -> Vec<Token>{
        while self.pos != self.len {
            match self.ch {
                b' ' => { },
                b'\n' => { 
                    self.line += 1; 
                    self.linepos = 0;
                },

                b'(' => { self.tokens.push(Token { 
                    value: String::from('('), 
                    t_type: TType::ParenLeft, 
                    position: self.linepos, 
                    line: self.line }); 
                },

                b')' => { self.tokens.push(Token { 
                    value: String::from(')'), 
                    t_type: TType::ParenRight, 
                    position: self.linepos, 
                    line: self.line }); 
                },
            
                b'+' => { self.tokens.push(Token { 
                    value: String::from('+'), 
                    t_type: TType::Plus, 
                    position: self.linepos, 
                    line: self.line }); 
                },
                
                b'-' => { self.tokens.push(Token { 
                    value: String::from('-'),
                    t_type: TType::Minus,
                    position: self.linepos,
                    line: self.line }); 
                },

                b'*' => { self.tokens.push(Token { 
                    value: String::from('*'),
                    t_type: TType::Multi,
                    position: self.linepos,
                    line: self.line });
                },

                b'/' => { self.tokens.push(Token { 
                    value: String::from('/'),
                    t_type: TType::Div,
                    position: self.linepos,
                    line: self.line });
                },

                b'!' => {
                    match self.get_next() {
                        b'=' => {
                            self.tokens.push(Token {
                                value: String::from("!="),
                                t_type: TType::BangEqual,
                                position: self.linepos,
                                line: self.line
                            });
                            self.advance();
                        },

                        _ => self.tokens.push(Token {
                            value: String::from('!'),
                            t_type: TType::Bang,
                            position: self.linepos,
                            line: self.line
                        })
                    };
                },

                b'=' => {
                    match self.get_next() {
                        b'=' => {
                            self.tokens.push(Token {
                                value: String::from("=="),
                                t_type: TType::EqualEqual,
                                position: self.linepos,
                                line: self.line
                            });
                            self.advance();
                        },

                        _ => self.tokens.push(Token {
                                value: String::from('='),
                                t_type: TType::Equal,
                                position: self.linepos,
                                line: self.line
                        })
                    };
                },

                b'<' => {
                    match self.get_next() {
                        b'=' => {
                            self.tokens.push(Token {
                                value: String::from("<="),
                                t_type: TType::LessEqual,
                                position: self.linepos,
                                line: self.line
                            });
                            self.advance();
                        },

                        _ => self.tokens.push(Token {
                                value: String::from('<'),
                                t_type: TType::Less,
                                position: self.linepos,
                                line: self.line
                        })
                    };
                },

                b'>' => {
                    match self.get_next() {
                        b'=' => {
                            self.tokens.push(Token {
                                value: String::from(">="),
                                t_type: TType::GreaterEqual,
                                position: self.linepos,
                                line: self.line
                            });
                            self.advance();
                        },

                        _ => self.tokens.push(Token {
                            value: String::from('>'),
                            t_type: TType::Greater,
                            position: self.linepos,
                            line: self.line
                        })
                    };
                },

                b'0' ..= b'9' => { 
                    let token: Token = self.make_num();
                    self.tokens.push(token); 
                },
                c => { 
                    if c.is_ascii() || c == b'_' {
                        let token: Token = self.make_ident();
                        self.tokens.push(token);
                    } else {
                        println!("ERROR: Syntax Error") 
                    }
                }
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
