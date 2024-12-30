use ast::{name::Name, Identifier, Stmt};

use crate::{errors::ParseErrorKind, tokens::TokenKind};

use super::Parser;

impl<'src> Parser<'src> {
    pub fn parse_statement(&mut self) -> Stmt {
        let stmt_res = match self.current_token_kind() {
            TokenKind::Begin => self.parse_begin_statement(),
            TokenKind::Commit => self.parse_commit_statement(),
            TokenKind::Drop => self.parse_drop_statement(),
            TokenKind::Savepoint => self.parse_savepoint_statement(),
            TokenKind::Release => self.parse_release_statement(),
            TokenKind::Rollback => self.parse_rollback_statement(),
            _ => {
                println!("Tokenkind {}", self.current_token_kind());
                println!("Current Span {}", self.current_token_span());
                self.parse_invalid_statement()
            }
        };
        let stmt = match stmt_res {
            Ok(stmt) => stmt,
            Err(err) => {
                self.eat_until(TokenKind::Semicolon);
                Stmt::Invalid(err)
            }
        };
        if !self.eat(TokenKind::Semicolon) {
            self.add_error(ParseErrorKind::MissingSemicolon, self.current_token_span());
        };
        stmt

    }

    //
    pub fn parse_begin_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Begin);
        self.eat(TokenKind::Transaction);
        Ok(Stmt::Begin(ast::StmtBegin {
            span: self.node_span(start),
        }))
    }

    pub fn parse_commit_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Commit);
        self.eat(TokenKind::Transaction);
        Ok(Stmt::Commit(ast::StmtCommit {
            span: self.node_span(start),
        }))
    }

    pub fn parse_drop_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Drop);
        let kind: ast::DdlTargetKind;
        if self.eat(TokenKind::Table) {
            kind = ast::DdlTargetKind::Table;
        } else if self.eat(TokenKind::Index) {
            kind = ast::DdlTargetKind::Index;
        } else {
            self.add_error(ParseErrorKind::InvalidDropTarget, self.current_token_span());
            self.tokens.skip_bump(TokenKind::Semicolon);
            return Err(ast::StmtInvalid {span: self.node_span(start)});
        };
        let mut exist_check = false;
        if self.eat(TokenKind::If) {
            if !self.expect(TokenKind::Exists){
                return Err(ast::StmtInvalid {span: self.node_span(start)});
            };
            exist_check = true;
        }
        let id = self.parse_identifier()
            .map_err(|error| {
            self.add_error(error.kind, error.span);
            ast::StmtInvalid {span: self.node_span(start)}
        })?;

        Ok(Stmt::Drop(ast::StmtDrop {
            span: self.node_span(start),
            kind,
            exist_check,
            id,
        }))
    }

    pub fn parse_release_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Release);
        self.eat(TokenKind::Savepoint);
        let id = self.parse_identifier()
            .map_err(|error| {
            self.add_error(error.kind, error.span);
            ast::StmtInvalid {span: self.node_span(start)}
        })?;
        Ok(Stmt::Release(ast::StmtRelease {
            span: self.node_span(start),
            id,
        }))
    }

    pub fn parse_rollback_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Rollback);
        self.eat(TokenKind::Transaction);
        let id: Option<Identifier>;
        if self.eat(TokenKind::To) {
            self.eat(TokenKind::Savepoint);
            id = Some(self.parse_identifier()
                .map_err(|error| {
                self.add_error(error.kind, error.span);
                ast::StmtInvalid {span: self.node_span(start)}
            })?);
        } else {
            id = None;
        }
        Ok(Stmt::Rollback(ast::StmtRollback {
            span: self.node_span(start),
            id,
        }))
    }

    pub fn parse_savepoint_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Savepoint);
        let id = self.parse_identifier()
            .map_err(|error| {
            self.add_error(error.kind, error.span);
            ast::StmtInvalid {span: self.node_span(start)}
        })?;
        Ok(Stmt::Savepoint(ast::StmtSavepoint {
            span: self.node_span(start),
            id,
        }))
    }

    pub fn parse_invalid_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        // Bump whatever invalid token started this invalid statement
        self.bump_any();
        self.add_error(
            ParseErrorKind::UnexpectedToken { found: self.current_token_kind() },
            self.node_span(start)
        );
        self.eat_until(TokenKind::Semicolon);
        Err(ast::StmtInvalid {
            span: self.node_span(start),
        })
    }
}

#[cfg(test)]
mod test {
    use ast::name::Name;
    use source_index::{location::Location, span::Span};

    use super::*;

    #[test]
    fn begin_stmt() {
        let source = "BEGIN;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(5));
        assert_eq!(
            stmt,
            Stmt::Begin(ast::StmtBegin {
                span: expected_span
            })
        );
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
        let expected_span = Span::new(Location::new(0), Location::new(9));
        assert_eq!(
            stmt,
            Stmt::Invalid(ast::StmtInvalid {
                span: expected_span
            })
        );
        assert!(parser.errors.first().is_some_and(|first| matches!(
            first.kind,
            ParseErrorKind::ExpectedIdentifier { found: _ }
        )));
    }

    #[test]
    fn savepoint_stmt() {
        let source = "SAVEPOINT s1;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(12));
        let expected_id_span = Span::new(Location::new(10), Location::new(12));
        let expected_id = ast::Identifier::new(Name::new("s1".to_string()), expected_id_span);
        assert!(parser.errors.len() == 0);
        assert_eq!(
            stmt,
            Stmt::Savepoint(ast::StmtSavepoint {
                span: expected_span,
                id: expected_id
            })
        );
    }

    #[test]
    fn release_stmt() {
        let source = "RELEASE s1;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(10));
        let expected_id_span = Span::new(Location::new(8), Location::new(10));
        let expected_id = ast::Identifier::new(Name::new("s1".to_string()), expected_id_span);
        assert_eq!(
            stmt,
            Stmt::Release(ast::StmtRelease {
                span: expected_span,
                id: expected_id
            })
        );
        let source_opt = "RELEASE SAVEPOINT s1;";
        parser = Parser::new(source_opt);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(20));
        let expected_id_span = Span::new(Location::new(18), Location::new(20));
        let expected_id = ast::Identifier::new(Name::new("s1".to_string()), expected_id_span);
        assert_eq!(
            stmt,
            Stmt::Release(ast::StmtRelease {
                span: expected_span,
                id: expected_id
            })
        );
    }

    #[test]
    fn rollback_stmt() {
        let source = "ROLLBACK;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(8));
        assert_eq!(
            stmt,
            Stmt::Rollback(ast::StmtRollback {
                span: expected_span,
                id: None,
            })
        );
        let source_opt = "ROLLBACK TRANSACTION TO SAVEPOINT s1;";
        parser = Parser::new(source_opt);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(36));
        let expected_id_span = Span::new(Location::new(34), Location::new(36));
        let expected_id = ast::Identifier::new(Name::new("s1".to_string()), expected_id_span);
        assert_eq!(
            stmt,
            Stmt::Rollback(ast::StmtRollback {
                span: expected_span,
                id: Some(expected_id),
            })
        );
    }

    #[test]
    fn drop_stmt() {
        let source = "DROP TABLE t1;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(13));
        let expected_id_span = Span::new(Location::new(11), Location::new(13));
        let expected_id = ast::Identifier::new(Name::new("t1".to_string()), expected_id_span);
        assert_eq!(
            stmt,
            Stmt::Drop(ast::StmtDrop {
                span: expected_span,
                kind: ast::DdlTargetKind::Table,
                exist_check: false,
                id: expected_id,
            })
        );
        let source = "DROP INDEX IF EXISTS i1;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(23));
        let expected_id_span = Span::new(Location::new(21), Location::new(23));
        let expected_id = ast::Identifier::new(Name::new("i1".to_string()), expected_id_span);
        assert_eq!(
            stmt,
            Stmt::Drop(ast::StmtDrop {
                span: expected_span,
                kind: ast::DdlTargetKind::Index,
                exist_check: true,
                id: expected_id,
            })
        );
    }

    #[test]
    fn drop_error() {
        let source = "DROP COLUMN t1;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(4));
        assert_eq!(
            stmt,
            Stmt::Invalid(ast::StmtInvalid {
                span: expected_span,
            })
        );
        assert_eq!(parser.errors.len(), 1);
    }
}
