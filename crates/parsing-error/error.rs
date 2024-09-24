use std::fmt;
use std::num::ParseIntError;
use std::error::Error;
use std::ops::Range;


#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: Range<usize>,
}


#[derive(Debug, PartialEq, Clone)]
pub enum ParseErrorType {
    InvalidInteger(ParseIntError),
    UnexpectedEOF,
    LexicalError
}

impl ParseError {
    pub fn new(location: Range<usize>, error: ParseErrorType) -> Self {
        ParseError {
            error,
            location,
        }
    }

    pub fn from_lexer(location: Range<usize>, lexical_error: LexicalError) -> Self {
        ParseError {
            location,
            error: match lexical_error {
                LexicalError::InvalidToken => ParseErrorType::LexicalError,
                LexicalError::InvalidInteger(err) => ParseErrorType::InvalidInteger(err)
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for LexicalError {}

