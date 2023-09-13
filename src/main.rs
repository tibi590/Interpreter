enum TokenType {
    String,
    Int,
    Bool,
    Add,
    Minus,
    Multiply,
    Divide,
    EOF
}

struct Token {
    token: String,
    tpe: TokenType,
    start_pos: i16,
}

struct Lexer {
    text: String,
    current_pos: i16,
}

fn main() {
    println!("Hello, world!");
}
