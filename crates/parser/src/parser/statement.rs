use ast::{Stmt, StmtBegin};

use crate::{errors::ParseErrorKind, tokens::TokenKind};

use super::Parser;

impl<'src> Parser<'src> {
    pub fn parse_statement(&mut self) -> Stmt {
        let stmt = match self.current_token_kind() {
            TokenKind::Begin => Stmt::Begin(self.parse_begin_statement()),
            TokenKind::Commit => Stmt::Commit(self.parse_commit_statement()),
            TokenKind::Drop => self.parse_drop_statement(),
            TokenKind::Savepoint => Stmt::Savepoint(self.parse_savepoint_statement()),
            _ => {
                print!("Tokenkind {}", self.current_token_kind());
                unimplemented!();
            }
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

    pub fn parse_drop_statement(&mut self) -> Stmt {
        let start = self.node_start();
        self.bump(TokenKind::Drop);
        match self.current_token_kind() {
            TokenKind::Table => {
                unimplemented!()
            }
            TokenKind::Index => {
                unimplemented!()
            }
            _ => {
                self.add_error(ParseErrorKind::InvalidDropTarget, self.current_token_span());
                self.tokens.skip_bump(TokenKind::Semicolon);
                return Stmt::Invalid(ast::StmtInvalid {span: self.node_span(start)})

            }

        }
    }

    pub fn parse_savepoint_statement(&mut self) -> ast::StmtSavepoint {
        let start = self.node_start();
        self.bump(TokenKind::Savepoint);
        let id = self.parse_identifier();
        ast::StmtSavepoint {
            span: self.node_span(start),
            id,
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
        assert!(matches!(stmt, Stmt::Begin(ast::StmtBegin { span: _ })));
    }

    #[test]
    fn missing_semicolon_error() {
        let source = "BEGIN";
        let mut parser = Parser::new(source);
        let _stmt = parser.parse_statement();
        assert!(parser.errors.len() == 1);
        assert!(parser
            .errors
            .first()
            .is_some_and(|first| matches!(first.kind, ParseErrorKind::MissingSemicolon)));
    }

    #[test]
    fn commit_stmt() {
        let source = "COMMIT;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        assert!(matches!(stmt, Stmt::Commit(ast::StmtCommit { span: _ })));
    }

    #[test]
    fn savepoint_without_identifier_stmt() {
        let source = "SAVEPOINT;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        assert!(matches!(stmt, Stmt::Savepoint(ast::StmtSavepoint { span: _ , id: _})));
        assert!(parser.
            errors
            .first()
            .is_some_and(|first| matches!(first.kind, ParseErrorKind::ExpectedIdentifier { found: _ })));
    }

    #[test]
    fn savepoint_stmt() {
        let source = "SAVEPOINT s1;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        assert!(matches!(stmt, Stmt::Savepoint(ast::StmtSavepoint { span: _ , id: _})));
        assert!(parser.errors.len() == 0);
    }

    #[test]
    fn invalid_drop_stmt() {
        let source = "DROP EGGS ON FLOOR;
        BEGIN;";
        let mut parser = Parser::new(source);
        let invalid_stmt = parser.parse_statement();
        assert!(matches!(invalid_stmt, Stmt::Invalid(ast::StmtInvalid { span: _})));
        let next_token = parser.current_token_kind();
        assert!(matches!(next_token, TokenKind::Begin),
            "expected begin, got {}", next_token);
    }
}
