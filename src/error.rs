use std::fmt;

#[derive(Debug)]
pub struct Diagnostic {
    pub line: usize,
    pub col: usize,
    pub message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}:{}] {}", self.line, self.col, self.message)
    }
}

impl std::error::Error for Diagnostic {}
