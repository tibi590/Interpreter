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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    Bang,
    Equal,

    Less,
    Greater,

    LessEqual,
    GreaterEqual,

    BangEqual,
    EqualEqual,

    True,
    False,

    Nil,

    If,

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

            TType::Bang => write!(f, "Bang"),
            TType::Equal => write!(f, "Equal"),

            TType::Less => write!(f, "Less"),
            TType::Greater => write!(f, "Greater"),

            TType::LessEqual => write!(f, "LessEqual"),
            TType::GreaterEqual => write!(f, "GreaterEqual"),

            TType::BangEqual => write!(f, "BangEqual"),
            TType::EqualEqual => write!(f, "EqualEqual"),
            
            TType::True => write!(f, "True"),
            TType::False => write!(f, "False"),

            TType::Nil => write!(f, "Nil"),

            TType::If => write!(f, "If"),

            TType::EOF => write!(f, "<EOF>")
        }
    }
}

