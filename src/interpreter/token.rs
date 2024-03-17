#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub t_type: TType,
    pub position: usize,
    pub line: usize
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}| {}: {}", self.line, self.position, self.value, self.t_type)
    }
}

#[derive(Debug, Clone)]
pub enum TType {
    Plus,
    Minus,
    Multi,
    Div,

    Int,
    Float,

    Ident,

    ParenLeft,
    ParenRight,

    EOF,
}

impl std::fmt::Display for TType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TType::Plus => write!(f, "Plus"),
            TType::Minus => write!(f, "Minus"),
            TType::Multi => write!(f, "Multi"),
            TType::Div => write!(f, "Div"),

            TType::Int => write!(f, "Int"),
            TType::Float => write!(f, "Float"),

            TType::Ident => write!(f, "Identifier"),

            TType::ParenLeft => write!(f, "ParenLeft"),
            TType::ParenRight => write!(f, "ParenRight"),
            
            TType::EOF => write!(f, "<EOF>")
        }
    }
}

