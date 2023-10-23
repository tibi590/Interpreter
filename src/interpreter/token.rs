
#[derive(Debug)]
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

#[derive(Debug)]
pub enum TType {
    Plus,
    Minus,
    Multi,
    Div,

    String,
    Int,
    Float,
    Bool,

    Ident,

    Print,

    Bang,
    Assign,
    Less,
    Grater,
    Equal,
    NotEqual,
    LessEqual,
    GraterEqual,

    ParenLeft,
    ParenRight,
    CurlyLeft,
    CurlyRight,

    EOF,
}

impl std::fmt::Display for TType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TType::Plus => write!(f, "Plus"),
            TType::Minus => write!(f, "Minus"),
            TType::Multi => write!(f, "Multi"),
            TType::Div => write!(f, "Div"),

            TType::String => write!(f, "String"),
            TType::Int => write!(f, "Int"),
            TType::Float => write!(f, "Float"),
            TType::Bool => write!(f, "Bool"),

            TType::Ident => write!(f, "Identifier"),

            TType::Print => write!(f, "Print"),

            TType::Assign => write!(f, "Assignment"),
            TType::Equal => write!(f, "Equal"),
            TType::Bang => write!(f, "Bang"),
            TType::NotEqual => write!(f, "Not Equal"),
            TType::Less => write!(f, "Less"),
            TType::LessEqual => write!(f, "Less Equal"),
            TType::Grater => write!(f, "Grater"),
            TType::GraterEqual => write!(f, "Grater Equal"),

            TType::ParenLeft => write!(f, "ParenLeft"),
            TType::ParenRight => write!(f, "ParenRight"),
            TType::CurlyLeft => write!(f, "CurlyLeft"),
            TType::CurlyRight => write!(f, "CurlyRight"),
            
            TType::EOF => write!(f, "<EOF>")
        }
    }
}

