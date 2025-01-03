use crate::{errors::{ParseError, ParseErrorKind}, tokens::TokenKind};

use super::Parser;

impl<'src> Parser<'src> {

    pub(crate) fn parse_alter_table_action(&mut self) -> Result<ast::AlterTableAction, ParseError> {
        let start = self.node_start();
        let kind = match self.current_token_kind() {
            TokenKind::Rename => self.parse_alter_table_rename(),
            TokenKind::Add => self.parse_alter_table_add(),
            TokenKind::Drop => self.parse_alter_table_drop(),
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::UnexpectedToken {
                        found: self.current_token_kind() },
                    span: self.node_span(start)
                })
            }
        }?;
        Ok(ast::AlterTableAction {
            span: self.node_span(start),
            kind,
        })
    }

    fn parse_alter_table_rename(&mut self) -> Result<ast::AlterTableActionKind, ParseError> {
        let start = self.node_start();
        self.bump(TokenKind::Rename);
        if self.eat(TokenKind::To) {
            let id = self.parse_identifier()?;
            return Ok(ast::AlterTableActionKind::Rename(
                    ast::AlterTableRename {
                        span: self.node_span(start),
                        kind: ast::AlterTableRenameKind::Table(id)
                    }));
        }
        self.eat(TokenKind::Column);
        let id_from = self.parse_identifier()?;
        if !self.expect(TokenKind::To) {
            return Err(ParseError {
                kind: ParseErrorKind::ExpectedToken {
                    found: self.current_token_kind(),
                    expected: TokenKind::To,
                },
                span: self.node_span(start),
            });
        }
        let id_to = self.parse_identifier()?;
        Ok(ast::AlterTableActionKind::Rename(
                ast::AlterTableRename {
                    span: self.node_span(start),
                    kind: ast::AlterTableRenameKind::Column(id_from, id_to)
                })
        )
    }

    fn parse_alter_table_add(&mut self) -> Result<ast::AlterTableActionKind, ParseError> {
        let start = self.node_start();
        self.bump(TokenKind::Add);
        self.eat(TokenKind::Column);
        let column = self.parse_column_def()?;
        Ok(ast::AlterTableActionKind::Add(
                ast::AlterTableAdd {
                    span: self.node_span(start),
                    column,
                }
        ))
    }

    fn parse_alter_table_drop(&mut self) -> Result<ast::AlterTableActionKind, ParseError> {
        let start = self.node_start();
        self.bump(TokenKind::Drop);
        self.eat(TokenKind::Column);
        let id = self.parse_identifier()?;
        Ok(ast::AlterTableActionKind::Drop(
                ast::AlterTableDrop {
                    span: self.node_span(start),
                    id,
                }
        ))
    }

    fn parse_column_def(&mut self) -> Result<ast::ColumnDef, ParseError> {
        let start = self.node_start();
        let id = self.parse_identifier()?;
        let type_name = self.parse_type_name()?;
        Ok(ast::ColumnDef {
            span: self.node_span(start),
            id,
            type_name,
        })
    }

    fn parse_type_name(&mut self) -> Result<ast::TypeName, ParseError> {
        let start = self.node_start();
        let external_type = match self.current_token_kind() {
            TokenKind::Char => {
                self.bump(TokenKind::Char);
                ast::ExternalType::Char
            }
            TokenKind::Integer => {
                self.bump(TokenKind::Integer);
                ast::ExternalType::Integer
            }
            TokenKind::Numeric => {
                self.bump(TokenKind::Numeric);
                ast::ExternalType::Numeric
            }
            TokenKind::Serial => {
                self.bump(TokenKind::Serial);
                ast::ExternalType::Serial
            }
            TokenKind::Varchar => {
                self.bump(TokenKind::Varchar);
                ast::ExternalType::Varchar
            }
            _ => {
                return Err(ParseError {
                    span: self.node_span(start),
                    kind: ParseErrorKind::ExpectedType { found: self.current_token_kind() }
                })
            }
        };
        if self.at(TokenKind::LParen) {
            let number_field = Some(self.parse_type_name_number_field()?);
            return Ok(ast::TypeName {
                span: self.node_span(start),
                external_type,
                number_field,
            })
        }
        Ok(ast::TypeName {
            span: self.node_span(start),
            external_type,
            number_field: None
        })
    }

    fn parse_type_name_number_field(&mut self) -> Result<ast::TypeNameNumberField, ParseError> {
        let start = self.node_start();
        self.bump(TokenKind::LParen);
        let first = self.parse_signed_number()?;
        let mut second = None;
        if self.eat(TokenKind::Comma) {
            second = Some(self.parse_signed_number()?);
        }
        if !self.eat(TokenKind::RParen) {
            return Err(ParseError {
                span: self.node_span(start),
                kind: ParseErrorKind::ExpectedToken { 
                    found: self.current_token_kind(),
                    expected: TokenKind::RParen }
            })
        }
        Ok(ast::TypeNameNumberField {
            span: self.node_span(start),
            first,
            second,
        })
    }

}
