use std::ops::Range;

use parsing_error::{ParseError, ParseErrorType};
use parsing_lexer::lexer::Lexer;
use parsing_lexer::token::Token;
use parsing_ast::{Script, StmtList};


pub(crate) struct Parser<'src> {
   source: &'src str, 

   // Iterator for tokens
   tokens: Lexer<'src>,

   current_token: Token,

   current_span: Range<usize>,


   errors: Vec<ParseError>
}

impl<'src> Parser<'src> {

    pub(crate) fn new(source: &'src str) -> Self {
        Parser {
            source,
            tokens: Lexer::new(source),
            errors: Vec::new()
        }
    }

    // Consumes Parser and returns Parsed
    pub(crate) fn parse(mut self) -> Parsed<Script> {
        let syntax = Script::Script(self.parse_stmt_list());
        self.finish(syntax)
    }

    fn parse_stmt_list(&mut self) -> StmtList {
        let body = self.parse_list_into_vec(
            Parser::parse_statement,
        );
        unimplemented!();
    }

    fn finish(self, syntax: Script) -> Parsed<Script> {
        unimplemented!();
    }

    fn parse_list_into_vec<T>(
        &mut self,
        parse_element: impl Fn(&mut Parser<'src>) -> T,
    ) -> Vec<T> {
        let mut elements = Vec::new();
        self.parse_list(|p| elements.push(parse_element(p)));
        elements
    }

    fn parse_list(
        &mut self,
        mut parse_element: impl FnMut(&mut Parser<'src>),
    ) {
        unimplemented!()
    }

    pub fn span(&self) -> Range<usize> {
        self.tokens.span()
    }

    // Consume current token if it matches kind. Return True if matches and eaten, 
    // otherwise return false
    fn eat(&mut self, kind: Token) -> bool {
        if 
    }


    // Bumps the current token assuming it is the given kind.
    //
    // # PANIC
    //
    // If it does not match, panics
    fn bump(&mut self, kind: Token) {
        assert_eq!(self.current_token, kind);

        self.do_bump(kind);
    }

}


#[derive(Debug, PartialEq, Clone)]
pub struct Parsed<T> {
    syntax: T,
    tokens: Vec<Token>,
    errors: Vec<ParseError>
}

