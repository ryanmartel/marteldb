use ast::{Stmt, Stmts};
use source_index::location::Location;
use source_index::span::{Span, Spanned};

use crate::errors::{ParseError, ParseErrorKind};

use crate::token_source::TokenSource;
use crate::tokens::{TokenKind, TokenValue};
use crate::{Parsed, Tokens};

mod expression;
mod statement;

pub struct Parser<'src> {
    // source string
    source: &'src str,

    tokens: TokenSource<'src>,

    errors: Vec<ParseError>,

    prev_token_end: Location,

    start_offset: Location,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        let tokens = TokenSource::new(source);

        Self {
            source,
            errors: Vec::new(),
            tokens,
            prev_token_end: Location::new(0),
            start_offset: Location::new(0),
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
        assert_eq!(self.current_token_kind(), kind);
        self.prev_token_end = self.current_token_span().end();
        self.tokens.bump(kind);
    }

    fn bump_any(&mut self) {
        self.prev_token_end = self.current_token_span().end();
        self.tokens.bump_any();
    }

    /// Take the token value from token source and bump the current token.
    fn bump_value(&mut self, kind: TokenKind) -> TokenValue {
        let value = self.tokens.take_value();
        self.bump(kind);
        value
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

    fn expect(&mut self, kind: TokenKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.add_error(
            ParseErrorKind::ExpectedToken { 
                found: self.current_token_kind(),
                expected: kind 
            },
            self.current_token_span()
        );
        return false;
    }

    // consume tokens until the current kind is found.
    // stops before eating stop_kind
    fn eat_until(&mut self, stop_kind: TokenKind) {
        self.tokens.skip_bump(stop_kind);
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.current_token_kind() == kind
    }

    pub fn parse(mut self) -> Parsed {
        let stmts = self.parse_stmts();

        assert_eq!(
            self.current_token_kind(),
            TokenKind::EndOfFile,
            "Parser should be at end of file!"
        );

        let errors = self.errors;
        let tokens = self.tokens.finish();

        return Parsed {
            stmts,
            tokens: Tokens::new(tokens),
            errors,
        };
    }

    fn parse_stmts(&mut self) -> Stmts {
        let body = self.parse_list_into_vec(Parser::parse_statement);
        self.bump(TokenKind::EndOfFile);
        Stmts {
            body,
            span: Span::new(self.start_offset, self.current_token_span().end()),
        }
    }

    fn parse_list_into_vec(
        &mut self,
        parse_element: impl Fn(&mut Parser<'src>) -> Stmt,
    ) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.parse_list(|p| stmts.push(parse_element(p)));
        stmts
    }

    fn parse_list(&mut self, mut parse_element: impl FnMut(&mut Parser<'src>)) {
        loop {
            if self.at(TokenKind::EndOfFile) {
                break;
            }
            parse_element(self);
        }
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
        assert!(
            matches!(parser.current_token_kind(), TokenKind::Semicolon),
            "should be semicolon, got {}",
            parser.current_token_kind()
        );
    }
}
