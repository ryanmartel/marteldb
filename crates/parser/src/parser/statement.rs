use ast::{Stmt, StmtBegin};

use crate::tokens::TokenKind;

use super::Parser;


impl<'src> Parser<'src> {

    pub fn parse_statement(&mut self) -> Stmt {
        match self.current_token_kind() {
            TokenKind::Begin => Stmt::Begin(self.parse_begin_statement()),
            _ => unimplemented!()
        }
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
}
