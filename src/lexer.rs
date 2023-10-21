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

    pub fn tokenize(mut self) -> Vec<Token>{
        while self.pos < self.text.len()-1 {
            if self.char == ' ' {
                self.advance();
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
            TType::PARENLEFT => write!(f, "PARENLEFT"),
            TType::PARENRIGHT => write!(f, "PARENRIGHT"),
            TType::CURLYLEFT => write!(f, "CURLYLEFT"),
            TType::CURLYRIGHT => write!(f, "CURLYRIGHT"),
            TType::EOF => write!(f, "<EOF>")
        }
    }
}

