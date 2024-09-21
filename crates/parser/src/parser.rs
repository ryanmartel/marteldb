use std::{error::Error, fmt::Display, iter::Peekable};

use logos::{Lexer, Logos, Span};


use super::ast;
use lexer::token::{LexicalError, Token};

pub struct ScriptParser<'input> {
    lexer: Lexer<'input, Token>,
    statements: Vec<ast::Stmt>,
}

impl<'input> ScriptParser<'input> {
    pub fn new(input: &'input str) -> Self {
        ScriptParser {
            lexer: Token::lexer(input),
            statements: Vec::new(),
        }
    }
    
    // <Stmt> :=  SELECT <SelectStmt> ;
    //          | INSERT <InsertStmt> ;
    //          | CREATE <CreateStmt> ;
    pub fn parse(mut self) -> Result<Vec<ast::Stmt>, ParseError> {
        // let mut lexer = self.lexer.peekable();
        while let Some(token) = self.lexer.next() {
            let span = self.lexer.span();
            match token {
                Ok(Token::Select) => {
                    // Check for Distinct
                    let select_stmt = if let Some(token) = self.lexer.next() {
                        match token {
                            Ok(Token::Distinct) => {
                                Box::new(parse_select_stmt(None, &mut self.lexer));
                            }
                            Err(err) => {
                                return Err(ParseError::LexingError(err));
                            }
                            _ => {
                                Box::new(parse_select_stmt(Some(token), &mut self.lexer));
                            }
                        }
                    }
                    // let select_stmt = Box::new(parse_select_stmt(&mut self.lexer)?);
                    // consume ; token
                    let mut e = 0;
                    if let Some(end_token) = self.lexer.next() {
                        let end_span = self.lexer.span();
                        match end_token {
                            Ok(Token::Semicolon) => {
                                e = end_span.end;
                            }
                            _ => return Err(ParseError::UnrecognizedToken { token: (span.start, token.unwrap(), span.end) })
                        }
                    }
                    self.statements.push(
                        ast::Stmt {
                            begin: span.start,
                            kind: ast::StmtKind::Select(select_stmt),
                            end: e
                    }
                    )
                }
                Ok(Token::Insert) => {println!("INSERT")}
                Ok(Token::Create) => {println!("CREATE")}
                _ => return Err(ParseError::UnrecognizedToken{
                    token: (span.start, token.unwrap(), span.end)
                })

            }
        }
        Ok(self.statements)
    }
}

// <SelectStmt> :=  (DISTINCT)? <ResultColList> FROM <TableList> (<WhereClause>)?
fn parse_select_stmt(current_token: Option<Result<Token, LexicalError>>, lexer: &mut Lexer<Token>) -> Result<ast::SelectStmt, ParseError> {
    let mut distinct = false;
    // Distinct?
    if current_token.is_none() {
        distinct = true;
    }
    let result_cols = parse_result_col_list(current_token, lexer)?;
    Ok(ast::SelectStmt {
        distinct,
        results: result_cols
    })
}
//
// // <ResultColList> := <ResultCol> (, ResultCol)?*
// // <ResultCol> :=  *
// //               | tableAll   -- table.*
// //               | tableCol   -- table.col
// //               | ident      -- col
fn parse_result_col_list(current_token: Option<Result<Token, LexicalError>>, lexer: &mut Lexer<Token>) -> Result<Vec<ast::ResultCol>, ParseError> {
    let mut current_ready = false;
    let mut result_cols = Vec::new();
    let mut awaits_comma = false;
    let mut awaits_col = false;
    let mut empty = true;
    if current_token.is_some() {
        current_ready = true;
    }
    // If there is a carryover token, consume that first
    while let Some(spanned_token) = if current_ready {current_ready = false; current_token} else {lexer.next()} {
        match spanned_token {
            Ok((left, token, right)) => {
                match token {
                    Token::Asterisk if !awaits_comma => {
                        result_cols.push(ast::ResultCol{
                            kind: ast::ResultColKind::All(None),
                        });
                        awaits_col = false;
                        empty = false;
                    }
                    Token::TableAll(table_all) if !awaits_comma => {
                        let split_index = table_all.find('.').unwrap();
                        let (table, _all) = table_all.split_once('.').unwrap();
                        result_cols.push(ast::ResultCol {
                            kind: ast::ResultColKind::All(Some(
                                          ast::Ident {
                                              begin : left,
                                              end : split_index + left,
                                              name : table.to_string()
                                          }
                                  ))
                        });
                        awaits_col = false;
                        empty = false;
                    }
                    Token::TableCol(table_col) if !awaits_comma => {
                        let split_index = table_col.find('.').unwrap();
                        let (table, col) = table_col.split_once('.').unwrap();
                        result_cols.push(ast::ResultCol {
                            kind: ast::ResultColKind::TableColumn(
                                      ast::TableColumn {
                                          table: Some(ast::Ident {
                                              begin: left,
                                              end: left + split_index,
                                              name: table.to_string()
                                          }),
                                          column: ast::Ident {
                                              begin: split_index+1,
                                              end: right,
                                              name: col.to_string()
                                          }
                                      }
                                  )
                        });
                        awaits_col = false;
                        empty = false;
                    }
                    Token::Identifier(iden) if !awaits_comma => {
                        result_cols.push(ast::ResultCol {
                            kind: ast::ResultColKind::TableColumn(
                                      ast::TableColumn {
                                          table: None,
                                          column: ast::Ident {
                                              begin: left,
                                              end: right,
                                              name: iden.to_string()
                                          }
                                      }
                                  )
                        });
                        awaits_col = false;
                        empty = false;
                    }
                    Token::Comma if awaits_comma => awaits_col = true,
                    _ => {
                        if empty {
                            return Err(ParseError::UnrecognizedToken{token: (left, token, right)})
                        }
                        return Ok(result_cols);
                    }

                }
            }
            _ => {}
        }
        awaits_comma = !awaits_col;
    }
    Ok(result_cols)

}

#[derive(Debug)]
pub enum ParseError {

    UnexpectedEof { location: usize},
    UnrecognizedToken{ token: (usize, Token, usize)},
    LexingError(LexicalError)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}
