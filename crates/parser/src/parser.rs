use source_index::location::Location;
use source_index::span::Span;

use crate::errors::ParseError;

use crate::token_source::TokenSource;
use crate::tokens::TokenKind;

mod statement;

pub struct Parser<'src> {
    // source string
    source: &'src str,

    tokens: TokenSource<'src>,

    errors: Vec<ParseError>,

    prev_token_end: Location,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str) -> Self {
        let tokens = TokenSource::new(source);

        Self {
            source,
            errors: Vec::new(),
            tokens,
            prev_token_end: Location::new(0),
        }
    }

    fn node_start(&self) -> Location {
        self.tokens.current_span().start()
    }

    fn node_span(&self, start: Location) -> Span {
        if self.prev_token_end <= start {
            Span::empty(start)
        } else {
            Span::new(start, self.prev_token_end)
        }
    }

    fn current_token_kind(&self) -> TokenKind {
        self.tokens.current_token_kind()
    }

    fn bump(&mut self, kind: TokenKind) {
        self.tokens.bump(kind);
    }

    fn peek(&mut self) -> TokenKind {
        self.tokens.peek()
    }

    // consume the current token if it is of given kind.
    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.bump(kind);
            true
        } else {
            false
        }
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.current_token_kind() == kind
    }
}
