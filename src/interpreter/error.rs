pub struct Error {
    pub file_location: String,
    pub value: String,
    pub pos: usize,
    pub line: usize,
    pub e_type: ErrorType
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset: String = " ".repeat(self.pos);
        let text: String = String::from(self.value.trim_end_matches('\n'));
        write!(f, "\nError type: {}; Location: {} at {}:{}\n|{}\n|{}^", self.e_type, self.file_location, self.line, self.pos, text, offset)
    }
}

pub enum ErrorType {
    SyntaxError,
    StringError
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::SyntaxError => write!(f, "SyntaxError"),
            ErrorType::StringError => write!(f, "StringError")
        }
    }
}
