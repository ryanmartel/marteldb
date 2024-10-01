use source_index::span::Span;

use crate::{lexer::Lexer, tokens::{Token, TokenKind}};

pub struct TokenSource<'src> {
    // underlying lexer for the tokens
    lexer: Lexer<'src>,

    // vector containing all tokens after parser has finished.
    tokens: Vec<Token>
}

impl<'src> TokenSource<'src> {
    
    pub fn new(source: &'src str) -> Self {
        let lexer = Lexer::new(source);
        let mut token_source = Self{
            lexer,
            tokens: Vec::new(),
        };
        // Move to first token.
        token_source.do_bump();
        token_source
    }

    // Bump the lexer to the next non-comment token.
    fn do_bump(&mut self) {
        loop {
            let kind = self.lexer.next_token();
            if matches!(kind, TokenKind::Comment) {
                self.tokens.push(Token::new(kind, self.current_span()));
                continue;
            }
            break;
        }

    }

    fn current_span(&self) -> Span {
        self.lexer.current_span()
    }
}
