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
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexical Error {:?}, at ({:?})", self.kind, self.span);
        Ok(())
    }
}

#[derive(Default, Debug)]
pub enum LexicalErrorKind {
    #[default]
    InvalidToken,
}
