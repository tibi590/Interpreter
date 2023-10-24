use crate::interpreter::error;
use crate::interpreter::token::{Token, TType};

pub struct Lexer{
    pub file_location: String,
    pub text: String,
    pub line_text: Vec<String>,
    pub pos: usize,
    pub line: usize,
    pub line_pos: usize,
    pub char: char,
    pub tokens: Vec<Token>,
    pub has_error: bool,
    pub errors: Vec<error::Error>
}

impl Default for Lexer {
    fn default() -> Lexer {
        Lexer { 
            file_location: "<stdin>".to_string(),
            text: "".to_string(),
            line_text: vec![],
            pos: 0,
            line: 0,
            line_pos: 0,
            char: ' ',
            tokens: vec![],
            has_error: false,
            errors: vec![]
        }
    }
}

impl Lexer {
    fn advance(&mut self) {
        if self.char != '\n' {
            self.line_pos += 1;
        } 

        self.pos += 1;
        if self.is_at_end() {
            self.char = self.text.chars().nth(self.pos).unwrap();
        }
    }

    fn peek(&mut self, ch: char) -> bool {
        let next_pos: usize = self.pos + 1;
        return ch == self.text.chars().nth(next_pos).unwrap();
    }

    fn get_next(&mut self) -> char {
        let next_pos: usize = self.pos + 1;
        return self.text.chars().nth(next_pos).unwrap();
    }

    fn is_at_end(&mut self) -> bool {
        return self.pos < self.text.len();
    }

    fn make_num(&mut self) -> Token {
        let mut value: String = String::from(self.char);
        let start_pos: usize = self.line_pos;
        let start_line: usize = self.line;
        let mut num_of_dots: i8 = 0;
        let mut next_ch: char = self.get_next();

        while (next_ch.is_numeric() || next_ch == '.') && self.is_at_end() {
            self.advance();
            value.push(self.char);

            if self.char == '.' {
                num_of_dots += 1;
            }

            next_ch = self.get_next();
        }

        if num_of_dots > 0 {
            return Token { value: value, t_type: TType::Float, position: start_pos, line: start_line};
        } else {
            return Token { value: value, t_type: TType::Int, position: start_pos, line: start_line};
        }
    }

    fn make_str(&mut self) -> Token {
        let mut value: String = String::from(self.char);
        let start_pos: usize = self.line_pos;
        let start_line: usize = self.line;
        let mut next_ch: char = self.get_next();
        let mut run: bool = true;
        
        while run {
            if next_ch.is_alphanumeric() && self.is_at_end() {
                self.advance();
                value.push(self.char);

                next_ch = self.get_next();
            } else if next_ch == '"' {
                self.advance();
                value.push(self.char);

                run = false
            } else if !self.is_at_end() {
                self.errors.push(error::Error { 
                    file_location: self.file_location.clone(),
                    value: self.line_text[self.line].clone(),
                    pos: start_pos,
                    line: start_line,
                    e_type: error::ErrorType::StringError
                });

                self.has_error = true;
                run = false;
            }
        }

        return Token { value: value, t_type: TType::String, position: start_pos, line: start_line};
    }


    fn make_identifier(&mut self) -> Token {
        let mut value: String = String::from(self.char);
        let start_pos: usize = self.line_pos;
        let start_line: usize = self.line;
        let mut next_ch: char = self.get_next();
        let token: Token;

        while next_ch.is_alphanumeric() && self.is_at_end() {
            self.advance();
            value.push(self.char);

            next_ch = self.get_next();
        }

        match value.as_str() {
            "true" => token = Token { value: value, t_type: TType::Bool, position: start_pos, line: start_line },
            "false" => token = Token { value: value, t_type: TType::Bool, position: start_pos, line: start_line },
            "print" => token = Token { value: value, t_type: TType::Print, position: start_pos, line: start_line },
            "if" => token = Token { value: value, t_type: TType::If, position: start_pos, line: start_line },
            "else" => token = Token { value: value, t_type: TType::Else, position: start_pos, line: start_line },
            _ => token = Token { value: value, t_type: TType::Ident, position: start_pos, line: start_line },
        }

        return token;
    }

    pub fn tokenize(mut self) -> (Vec<error::Error>, Vec<Token>) {
        println!("{}", self.text);
        while self.is_at_end() {
            let ch: char = self.char;
            println!("'{}'", ch);
            
            match ch {
                ' ' | '\t' | '\r' => {},
                '\n' => { 
                    self.line += 1; 
                    self.line_pos = 0;
                },
                'A'..='Z' | 'a'..='z' | '_' => {
                    let token: Token = self.make_identifier();
                    self.tokens.push(token);
                },
                '0'..='9' => {
                    let token: Token = self.make_num();
                    self.tokens.push(token);
                },
                '"' => {
                    let token: Token = self.make_str();
                    self.tokens.push(token);
                },
                '+' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Plus, position: self.line_pos, line: self.line }),
                '-' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Minus, position: self.line_pos, line: self.line }),
                '*' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Multi, position: self.line_pos, line: self.line }),
                '/' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Div, position: self.line_pos, line: self.line }),
                '(' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::ParenLeft, position: self.line_pos, line: self.line }),
                ')' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::ParenRight, position: self.line_pos, line: self.line }),
                '{' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::CurlyLeft, position: self.line_pos, line: self.line }),
                '}' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::CurlyRight, position: self.line_pos, line: self.line }),
                '=' if self.peek('=') => {
                    self.tokens.push(Token { value: "==".to_string(), t_type: TType::Equal, position: self.line_pos, line: self.line });
                    self.advance();
                },
                '!' if self.peek('=') => {
                    self.tokens.push(Token { value: "!=".to_string(), t_type: TType::NotEqual, position: self.line_pos, line: self.line });
                    self.advance();
                },
                '<' if self.peek('=') => {
                    self.tokens.push(Token { value: "<=".to_string(), t_type: TType::LessEqual, position: self.line_pos, line: self.line });
                    self.advance();
                },
                '>' if self.peek('=') => {
                    self.tokens.push(Token { value: ">=".to_string(), t_type: TType::GraterEqual, position: self.line_pos, line: self.line });
                    self.advance();
                },
                '=' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Assign, position: self.line_pos, line: self.line }),
                '!' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Bang, position: self.line_pos, line: self.line }),
                '<' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Less, position: self.line_pos, line: self.line }),
                '>' => self.tokens.push(Token { value: ch.to_string(), t_type: TType::Grater, position: self.line_pos, line: self.line }),
                _ => {
                    println!("x{}x", ch);
                    self.errors.push(error::Error { 
                        file_location: self.file_location.clone(),
                        value: self.line_text[self.line].clone(),
                        pos: self.line_pos,
                        line: self.line,
                        e_type: error::ErrorType::SyntaxError
                    });
                    self.has_error = true;
                },
            }

            self.advance();
        }

        self.tokens.push(Token { value: "<EOF>".to_string(), t_type: TType::EOF, position: self.line_pos, line: self.line });

        return (self.errors, self.tokens);
    }
}

