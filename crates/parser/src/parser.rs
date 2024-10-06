use source_index::location::Location;
use source_index::span::{Span, Spanned};

use crate::errors::{ParseError, ParseErrorKind};

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

    fn current_token_span(&self) -> Span {
        self.tokens.current_span()
    }

    fn add_error<T>(&mut self, error: ParseErrorKind, span: T)
    where
        T: Spanned,
    {
        let is_same_location = self
            .errors
            .last()
            .is_some_and(|last| last.span.start() == span.start());
        if !is_same_location {
            self.errors.push(ParseError {
                kind: error,
                span: span.span(),
            })
        }
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

    // consume tokens until the current kind is found. 
    // stops before eating stop_kind
    fn eat_until(&mut self, stop_kind: TokenKind) {
        self.tokens.skip_bump(stop_kind);
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.current_token_kind() == kind
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eof_ending() {
        let source = "";
        let mut parser = Parser::new(source);
        assert!(parser.eat(TokenKind::EndOfFile));
    }

    #[test]
    fn skip_bump_test() {
        let source = "BEGIN BEGIN BEGIN BEGIN;
        BEGIN;";
        let mut parser = Parser::new(source);
        parser.bump(TokenKind::Begin);
        parser.eat_until(TokenKind::Semicolon);
        assert!(matches!(parser.current_token_kind(), TokenKind::Semicolon),
            "should be semicolon, got {}", parser.current_token_kind());
    }
}
