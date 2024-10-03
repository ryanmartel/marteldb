use std::{fmt::{Display, Write}, ops::Deref};

use source_index::span::Span;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,

    pub span: Span,
}

impl ParseError {

    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self {
            kind,
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    Lexical(LexicalErrorKind),
    MissingSemicolon,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

    span: Span
}

impl LexicalError {

    pub fn new(kind: LexicalErrorKind, span: Span) -> Self {
        Self {
            kind,
            span,
        }
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
            LexicalErrorKind::UnterminatedString => {
                f.write_str("Unterminated string literal")
            }
            LexicalErrorKind::InvalidInt => {
                f.write_str("Integer format not supported or is too large")
            }
            LexicalErrorKind::InvalidFloat => {
                f.write_str("Float format not supported or is too large")
            }
            LexicalErrorKind::InvalidNumber => {
                f.write_str("This number format is not supported")
            }
            LexicalErrorKind::InvalidToken => {
                f.write_str("Unexpected or invalid token found")
            }
        }
    }
}
