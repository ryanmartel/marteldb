use std::fmt::Display;

use source_index::span::Span;

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
        write!(f, "Lexical Error {:?}, at ({:?})", self.kind, self.span);
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
