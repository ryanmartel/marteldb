use std::{error::Error, fmt::Display, iter::Peekable};

use super::ast;
use lexer::{lexer::Lexer, token::{LexicalError, Token}};

pub struct ScriptParser {
    statements: Vec<ast::Stmt>,
}

impl ScriptParser {
    pub fn new() -> Self {
        ScriptParser {
            statements: Vec::new(),
        }
    }
    
    // <Stmt> :=  SELECT <SelectStmt> ;
    //          | INSERT <InsertStmt> ;
    //          | CREATE <CreateStmt> ;
    pub fn parse(mut self, lexer: Lexer) -> Result<Vec<ast::Stmt>, ParseError> {
        let mut lexer = lexer.peekable();
        while let Some(spanned_token) = lexer.next() {
            match spanned_token {
                Ok((left, token, right)) => {
                    match token {
                        Token::Select => {
                            let select_stmt = Box::new(parse_select_stmt(&mut lexer)?);
                            // consume ; token
                            let mut e = 0;
                            if let Some(st) = lexer.next() {
                                match st {
                                    Ok((_left, Token::Semicolon, right)) => {
                                        e = right
                                    }
                                    _ => return Err(ParseError::UnrecognizedToken { token: (left, token, right) })
                                }
                            }
                            self.statements.push(
                                ast::Stmt {
                                    begin: left,
                                    kind: ast::StmtKind::Select(select_stmt),
                                    end: e
                                }
                            )
                            // println!("SELECT");
                            // parse_select_stmt(&mut lexer);
                        }
                        Token::Insert => {println!("INSERT")}
                        Token::Create => {println!("CREATE")}
                        _ => return Err(ParseError::UnrecognizedToken{
                            token: (left, token, right)
                        })

                    }
                }
                _ => {}
            }
        }
        Ok(self.statements)
    }
}

// <SelectStmt> :=  (DISTINCT)? <ResultColList> FROM <TableList> (<WhereClause>)?
fn parse_select_stmt(lexer: &mut Peekable<Lexer>) -> Result<ast::SelectStmt, ParseError> {
    let mut distinct = false;
    // Distinct?
    if let Some(spanned_token) = lexer.peek() {
        match spanned_token {
            Ok((_left, token, _right)) => {
                match token {
                    Token::Distinct => {
                        distinct = true;
                        lexer.next();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    let result_cols = parse_result_col_list(lexer)?;
    Ok(ast::SelectStmt {
        distinct: distinct,
        results: result_cols
    })
}

// <ResultColList> := <ResultCol> (, ResultCol)?*
// <ResultCol> :=  *
//               | tableAll   -- table.*
//               | tableCol   -- table.col
//               | ident      -- col
fn parse_result_col_list(lexer: &mut Peekable<Lexer>) -> Result<Vec<ast::ResultCol>, ParseError> {
    let mut result_cols = Vec::new();
    let mut awaits_comma = false;
    let mut awaits_col = false;
    let mut empty = true;
    while let Some(spanned_token) = lexer.next() {
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
