use std::{
    fmt::{Display, Write},
    ops::Deref,
};

use source_index::span::Span;

use crate::tokens::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,

    pub span: Span,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseErrorKind {
    ExpectedIdentifier {
        found: TokenKind,
    },
    ExpectedNumeric {
        found: TokenKind,
    },
    ExpectedToken {
        found: TokenKind,
        expected: TokenKind,
    },
    ExpectedType {
        found: TokenKind,
    },
    ExpectedValue,
    UnexpectedToken {
        found: TokenKind,
    },
    InvalidDropTarget,
    Lexical(LexicalErrorKind),
    MissingSemicolon,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::ExpectedIdentifier { found } => {
                write!(f, "Expected an identifier, found {found} instead.")
            }
            ParseErrorKind::ExpectedNumeric { found } => {
                write!(f, "Expected Numeric, found {found} instead.")
            }
            ParseErrorKind::ExpectedToken { found, expected } => {
                write!(f, "Expected {expected}, Found {found}")
            }
            ParseErrorKind::ExpectedType { found } => {
                write!(f, "Expected A Type Token, Found {found}")
            }
            ParseErrorKind::ExpectedValue => {
                write!(f, "Expected a value to be attached to this token")
            }
            ParseErrorKind::UnexpectedToken { found } => {
                write!(f, "Unexpected Token {found}. Can not start statement")
            }
            ParseErrorKind::InvalidDropTarget => {
                f.write_str("Expected either TABLE or INDEX following DROP")
            }
            ParseErrorKind::MissingSemicolon => {
                f.write_str("Missing terminating semicolon at end of statement.")
            }
            ParseErrorKind::Lexical(ref lexical_error) => {
                write!(f, "{lexical_error}")
            }
        }
    }
}

#[derive(Debug)]
pub struct LexicalError {
    kind: LexicalErrorKind,

    span: Span,
}

impl LexicalError {
    pub fn new(kind: LexicalErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn error_kind(&self) -> LexicalErrorKind {
        self.kind
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexical Error {:?}, at ({:?})", self.kind, self.span)?;
        Ok(())
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum LexicalErrorKind {
    #[default]
    InvalidToken,

    InvalidNumber,
    InvalidFloat,
    InvalidInt,
    UnterminatedString,
}

impl Display for LexicalErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalErrorKind::UnterminatedString => f.write_str("Unterminated string literal"),
            LexicalErrorKind::InvalidInt => {
                f.write_str("Integer format not supported or is too large")
            }
            LexicalErrorKind::InvalidFloat => {
                f.write_str("Float format not supported or is too large")
            }
            LexicalErrorKind::InvalidNumber => f.write_str("This number format is not supported"),
            LexicalErrorKind::InvalidToken => f.write_str("Unexpected or invalid token found"),
        }
    }
}
