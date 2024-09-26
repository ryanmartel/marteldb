use std::fmt;
use std::num::ParseIntError;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: (u32, u32)
}


#[derive(Debug, PartialEq, Clone)]
pub enum ParseErrorType {
    InvalidInteger(ParseIntError),
    UnexpectedEOF,
    LexicalError
}

impl ParseError {
    pub fn new(location: (u32, u32), error: ParseErrorType) -> Self {
        ParseError {
            error,
            location,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct LexicalError {
    error: LexicalErrorType,
    location: (u32, u32)
}
#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalErrorType {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalErrorType {
    fn from(err: ParseIntError) -> Self {
        LexicalErrorType::InvalidInteger(err)
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for LexicalError {}

impl LexicalError {
    pub fn location(&self) -> (u32, u32) {
        self.location
    }
}
