pub struct Lexer{
    pub text: String,
    pub pos: usize,
    pub char: char,
    pub tokens: Vec<Token>
}

impl Lexer {
    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.text.len()-1 {
            self.char = self.text.chars().nth(self.pos).unwrap();
        }
    }

    fn make_num(&mut self) -> Token {
        let mut value: String = String::new();
        let start_pos: usize = self.pos;
        let mut num_of_dots: i8 = 0;

        while (self.char.is_numeric() || (self.char == '.' && num_of_dots <= 1)) && self.pos < self.text.len()-1 {
            if self.char == '.' {
                num_of_dots += 1;
            }
            value.push(self.char);
            self.advance();
        }

        if num_of_dots > 0 {
            return Token { value: value, t_type: TType::FLOAT, position: start_pos};
        } else {
            return Token { value: value, t_type: TType::INT, position: start_pos};
        }
    }

    fn make_str(&mut self) -> Token {
        let mut value: String = String::from(self.char);
        let start_pos: usize = self.pos;
        let mut run: bool = true;
        self.advance();

        while run {
            if self.char != '"' && self.pos < self.text.len()-1 {
                value.push(self.char);
                self.advance();
            } else if self.pos == self.text.len()-1 {
                println!("STRING NOT FINISHED");
                run = false;
            } else {
                run = false;
            }
        }    
        value.push(self.char);
        self.advance();

        return Token { value: value, t_type: TType::STRING, position: start_pos };
    }


    fn make_identifier(&mut self) -> Token {
        let mut value: String = String::new();
        let start_pos: usize = self.pos;

        while self.char.is_alphanumeric() && self.pos < self.text.len()-1 {
            value.push(self.char);
            self.advance();
        }

        return Token { value: value, t_type: TType::IDENT, position: start_pos};
    }

    pub fn tokenize(mut self) -> Vec<Token>{
        while self.pos < self.text.len()-1 {
            if self.char.is_whitespace() {
                self.advance();
                continue;
            } else if self.char.is_numeric() {
                let token: Token = self.make_num();
                self.tokens.push(token);
                continue;
            } else if self.char.is_alphabetic() {
                let token: Token = self.make_identifier();
                self.tokens.push(token);
                continue;
            } else if self.char == '"' {
                let token: Token = self.make_str();
                self.tokens.push(token);
                continue;
            }

            match self.char {
                '+' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::PLUS, position: self.pos }),
                '-' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::MINUS, position: self.pos }),
                '*' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::MULTI, position: self.pos }),
                '/' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::DIV, position: self.pos }),
                '(' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::PARENLEFT, position: self.pos }),
                ')' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::PARENRIGHT, position: self.pos }),
                '{' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::CURLYLEFT, position: self.pos }),
                '}' => self.tokens.push(Token { value: self.char.to_string(), t_type: TType::CURLYRIGHT, position: self.pos }),
                _ => println!("Syntax Error"),
            }

            self.advance();
        }

        self.tokens.push(Token { value: "<EOF>".to_string(), t_type: TType::EOF, position: self.pos });

        return self.tokens;
    }
}

#[derive(Debug)]
pub struct Token {
    value: String,
    t_type: TType,
    position: usize
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}| {}: {}", self.position, self.value, self.t_type)
    }
}

#[derive(Debug)]
enum TType {
    PLUS,
    MINUS,
    MULTI,
    DIV,
    STRING,
    INT,
    FLOAT,
    IDENT,
    ASSIGN,
    EQUAL,
    NOT_EQUAL,
    LESS,
    LESS_EQUAL,
    GRATER,
    GRATER_EQUAL,
    PARENLEFT,
    PARENRIGHT,
    CURLYLEFT,
    CURLYRIGHT,
    EOF,
}

impl std::fmt::Display for TType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TType::PLUS => write!(f, "PLUS"),
            TType::MINUS => write!(f, "MINUS"),
            TType::MULTI => write!(f, "MULTI"),
            TType::DIV => write!(f, "DIV"),
            TType::STRING => write!(f, "STRING"),
            TType::INT => write!(f, "INT"),
            TType::FLOAT => write!(f, "FLOAT"),
            TType::IDENT => write!(f, "IDENTIFIER"),
            TType::ASSIGN => write!(f, "ASSIGNMENT"),
            TType::EQUAL=> write!(f, "EQUAL"),
            TType::NOT_EQUAL=> write!(f, "NOT EQUAL"),
            TType::LESS => write!(f, "LESS"),
            TType::LESS_EQUAL => write!(f, "LESS EQUAL"),
            TType::GRATER => write!(f, "GRATER"),
            TType::GRATER_EQUAL => write!(f, "GRATER EQUAL"),
            TType::PARENLEFT => write!(f, "PARENLEFT"),
            TType::PARENRIGHT => write!(f, "PARENRIGHT"),
            TType::CURLYLEFT => write!(f, "CURLYLEFT"),
            TType::CURLYRIGHT => write!(f, "CURLYRIGHT"),
            TType::EOF => write!(f, "<EOF>")
        }
    }
}

