use ast::{name::Name, Identifier, Stmt};

use crate::{errors::ParseErrorKind, tokens::TokenKind};

use super::Parser;

impl<'src> Parser<'src> {
    pub fn parse_statement(&mut self) -> Stmt {
        let stmt_res = match self.current_token_kind() {
            TokenKind::Alter => self.parse_alter_table_statement(),
            TokenKind::Begin => self.parse_begin_statement(),
            TokenKind::Commit => self.parse_commit_statement(),
            TokenKind::Drop => self.parse_drop_statement(),
            TokenKind::Reindex => self.parse_reindex_statement(),
            TokenKind::Release => self.parse_release_statement(),
            TokenKind::Rollback => self.parse_rollback_statement(),
            TokenKind::Savepoint => self.parse_savepoint_statement(),
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

    pub fn parse_alter_table_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Alter);
        if !self.expect(TokenKind::Table) {
            return Err(ast::StmtInvalid {span: self.node_span(start)});
        };
        let id = self.parse_identifier()
            .map_err(|error| {
            self.add_error(error.kind, error.span);
            ast::StmtInvalid {span: self.node_span(start)}
        })?;
        let action = self.parse_alter_table_action()
            .map_err(|error| {
                self.add_error(error.kind, error.span);
                ast::StmtInvalid {span: self.node_span(start)}
            })?;
        Ok(Stmt::Alter(ast::StmtAlter {
            span: self.node_span(start),
            id,
            action,
        }))
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

    pub fn parse_create_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        unimplemented!()
    }

    pub fn parse_delete_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        unimplemented!()
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

    pub fn parse_insert_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        unimplemented!()
    }

    pub fn parse_reindex_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        let start = self.node_start();
        self.bump(TokenKind::Reindex);
        let id;
        if self.current_token_kind() == TokenKind::Name {
            id = Some(self.parse_identifier()
                .map_err(|error| {
                    self.add_error(error.kind, error.span);
                    ast::StmtInvalid {span: self.node_span(start)}
                })?);
        } else {
            id = None
        }
        Ok(Stmt::Reindex(ast::StmtReindex {
            span: self.node_span(start),
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

    pub fn parse_select_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        unimplemented!()
    }

    pub fn parse_update_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        unimplemented!()
    }

    pub fn parse_vacuum_statement(&mut self) -> Result<Stmt, ast::StmtInvalid> {
        unimplemented!()
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
    use ast::{name::Name, ColumnConstraintKind};
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
    fn alter_stmt_rename() {
        let source = "ALTER TABLE t1 RENAME COLUMN c1 TO c2;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(37));
        let rename_span = Span::new(Location::new(15), Location::new(37));
        let id_span = Span::new(Location::new(12), Location::new(14));
        let id = ast::Identifier::new(Name::new("t1".to_string()), id_span);
        let id_from_span = Span::new(Location::new(29), Location::new(31));
        let id_from = ast::Identifier::new(Name::new("c1".to_string()), id_from_span);
        let id_to_span = Span::new(Location::new(35), Location::new(37));
        let id_to = ast::Identifier::new(Name::new("c2".to_string()), id_to_span);
        assert_eq!(
            stmt,
            Stmt::Alter(ast::StmtAlter {
                span: expected_span,
                id,
                action: ast::AlterTableAction {
                    span: rename_span,
                    kind: ast::AlterTableActionKind::Rename(ast::AlterTableRename {
                        span: rename_span,
                        kind: ast::AlterTableRenameKind::Column(id_from,id_to)
                    })
                }
            })
        );
    }

    #[test]
    fn alter_stmt_add() {
        let source = "ALTER TABLE t1 ADD COLUMN c1 NUMERIC(7,4) UNIQUE ON CONFLICT ABORT;";
        let mut parser = Parser::new(source);
        let stmt = parser.parse_statement();
        let expected_span = Span::new(Location::new(0), Location::new(66));
        let add_span = Span::new(Location::new(15), Location::new(66));
        let id_span = Span::new(Location::new(12), Location::new(14));
        let id = ast::Identifier::new(Name::new("t1".to_string()), id_span);
        let id_col_span = Span::new(Location::new(26), Location::new(28));
        let id_col = ast::Identifier::new(Name::new("c1".to_string()), id_col_span);
        let column_def_span = Span::new(Location::new(26), Location::new(66));
        let type_name_span = Span::new(Location::new(29), Location::new(41));
        let number_field_span = Span::new(Location::new(36), Location::new(41));
        let first_number_span = Span::new(Location::new(37), Location::new(38));
        let second_number_span = Span::new(Location::new(39), Location::new(40));
        let constraint_span = Span::new(Location::new(42), Location::new(66));

        assert_eq!(
            stmt,
            Stmt::Alter(ast::StmtAlter {
                span: expected_span,
                id,
                action: ast::AlterTableAction {
                    span: add_span,
                    kind: ast::AlterTableActionKind::Add(ast::AlterTableAdd {
                        span: add_span,
                        column: ast::ColumnDef {
                            span: column_def_span,
                            id: id_col,
                            type_name: ast::TypeName {
                                span: type_name_span,
                                external_type: ast::ExternalType::Numeric,
                                number_field: Some(ast::TypeNameNumberField {
                                    span: number_field_span,
                                    first: ast::SignedNumber::Integer(ast::IntLiteral {
                                        span: first_number_span,
                                        value: 7
                                    }),
                                    second: Some(ast::SignedNumber::Integer(ast::IntLiteral {
                                        span: second_number_span,
                                        value: 4
                                    }))
                                })
                            },
                            constraint_list: ast::ColumnConstraintList {
                                span: constraint_span,
                                constraints: vec!(
                                    ast::ColumnConstraint {
                                        span: constraint_span,
                                        kind: ColumnConstraintKind::Unique(Some(
                                                ast::ConflictAction::Abort
                                        ))
                                    }
                                )
                            }
                        }
                    })
                }
            })
        )
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
