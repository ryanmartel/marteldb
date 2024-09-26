use crate::parser::Parser;
use parsing_ast:: {
    Stmt,
};
use parsing_lexer::token::TokenKind;

impl<'src> Parser<'src> {
    
    pub(super) fn parse_statement(&mut self) -> Stmt {
        let start = self.node_start();
        match self.current_token_kind() {
            TokenKind::Select => println!("SELECT"),
            _ => {}
        }
        unimplemented!()
    }
}
