use source_index::span::Span;

use crate::{
    lexer::Lexer,
    tokens::{Token, TokenKind},
};

pub struct TokenSource<'src> {
    // underlying lexer for the tokens
    lexer: Lexer<'src>,

    // vector containing all tokens after parser has finished.
    tokens: Vec<Token>,
}

impl<'src> TokenSource<'src> {
    pub fn new(source: &'src str) -> Self {
        let lexer = Lexer::new(source);
        let mut token_source = Self {
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

    pub fn bump(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(kind, self.current_span()));
        self.do_bump();
    }

    // bumps any token as if `[bump]` was called.
    // Stops on stop token
    pub fn skip_bump(&mut self, stop_token: TokenKind) {
        loop {
            let kind = self.lexer.next_token();
            if kind == stop_token {
                return;
            }
        }
    }

    pub fn peek(&mut self) -> TokenKind {
        let checkpoint = self.lexer.checkpoint();
        let next = self.next_non_comment_token();
        self.lexer.rewind(checkpoint);
        next
    }

    pub fn current_span(&self) -> Span {
        self.lexer.current_span()
    }

    pub fn current_token_kind(&self) -> TokenKind {
        self.lexer.current_kind()
    }

    fn next_non_comment_token(&mut self) -> TokenKind {
        loop {
            let kind = self.lexer.next_token();
            if matches!(kind, TokenKind::Comment) {
                continue;
            }
            break kind;
        }
    }
}
