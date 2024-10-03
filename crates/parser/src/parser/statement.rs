use ast::{Stmt, StmtBegin};

use crate::{errors::ParseErrorKind, tokens::TokenKind};

use super::Parser;


impl<'src> Parser<'src> {

    pub fn parse_statement(&mut self) -> Stmt {
        let stmt = match self.current_token_kind() {
            TokenKind::Begin => Stmt::Begin(self.parse_begin_statement()),
            TokenKind::Commit => Stmt::Commit(self.parse_commit_statement()),
            _ => unimplemented!()
        };
        if !self.eat(TokenKind::Semicolon) {
            self.add_error(ParseErrorKind::MissingSemicolon, self.current_token_span());
        }
        stmt
    }

    //
    pub fn parse_begin_statement(&mut self) -> ast::StmtBegin {
        let start = self.node_start();
        self.bump(TokenKind::Begin);
        self.eat(TokenKind::Transaction);
        ast::StmtBegin {
            span: self.node_span(start),
        }
    }

    pub fn parse_commit_statement(&mut self) -> ast::StmtCommit {
        let start = self.node_start();
        self.bump(TokenKind::Commit);
        self.eat(TokenKind::Transaction);
        ast::StmtCommit {
            span: self.node_span(start),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn begin_stmt() {
        let source = "BEGIN;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        assert!(matches!(stmt, Stmt::Begin(ast::StmtBegin{ span: _})));
    }

    #[test]
    fn missing_semicolon_error() {
        let source = "BEGIN";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        assert!(parser.errors.len() == 1);
        assert!(parser.errors.first()
            .is_some_and(|first| matches!(first.kind, ParseErrorKind::MissingSemicolon)));
    }

    #[test]
    fn commit_stmt() {
        let source = "COMMIT;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        assert!(matches!(stmt, Stmt::Commit(ast::StmtCommit{ span: _})));
    }
}
