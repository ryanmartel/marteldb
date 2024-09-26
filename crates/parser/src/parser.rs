use parsing_error::{ParseError, ParseErrorType};
use parsing_lexer::{lexer::Lexer, token::TokenKind};
use parsing_lexer::token::Token;
use parsing_ast::{Script, StmtList};


pub(crate) struct Parser<'src> {
   source: &'src str, 

   // Iterator for tokens
   tokens: Lexer<'src>,

   prev_token_end: u32,

   errors: Vec<ParseError>
}

impl<'src> Parser<'src> {

    pub(crate) fn new(source: &'src str) -> Self {
        Parser {
            source,
            tokens: Lexer::new(source),
            prev_token_end: 0,
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
        self.bump(TokenKind::EndOfFile);
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
        loop {
            parse_element(self);
        }
    }


    // Consume current token if it matches kind. Return True if matches and eaten, 
    // otherwise return false
    fn eat(&mut self, kind: Token) -> bool {
        unimplemented!()
    }


    // Bumps the current token assuming it is the given kind.
    //
    // # PANIC
    //
    // If it does not match, panics
    fn bump(&mut self, kind: TokenKind) {
        assert_eq!(self.current_token_kind(), kind);

        self.do_bump(kind);
    }

    // Move the parser to the next token
    fn do_bump(&mut self, kind:TokenKind) {
        if !matches!(
            self.current_token_kind(),
            // Dont include newlines in the body
            TokenKind::Newline
        ) {
            self.prev_token_end = self.current_token_range().1;
        }
        self.tokens.bump(kind);
    }

    pub(crate) fn current_token_kind(&self) -> TokenKind {
        self.tokens.current_kind()
    }

    fn current_token_range(&self) -> (u32, u32) {
        self.tokens.current_range()
    }

    pub(crate) fn node_start(&self) -> u32 {
        self.current_token_range().0
    }

}


#[derive(Debug, PartialEq, Clone)]
pub struct Parsed<T> {
    syntax: T,
    tokens: Vec<Token>,
    errors: Vec<ParseError>
}

