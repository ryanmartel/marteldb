use std::fmt::Display;

use source_index::{location::Location, span::Span};

pub struct Token {
    /// The kind of token.
    kind: TokenKind,
    /// span of the token.
    span: Span,
}

impl Token {

    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token {
            kind,
            span,
        }
    }

    pub fn start(&self) -> Location {
        self.span.start()
    }

    pub fn end(&self) -> Location {
        self.span.end()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TokenKind {

    Identifier,

    // Keywords -------------
    Select,


    // matches '('
    LParen,
    // matches ')'
    RParen,
    // matches ';'
    Semicolon,


    Unknown,
    EndOfFIle,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone)]
pub enum TokenValue {
    #[default]
    None,
}
