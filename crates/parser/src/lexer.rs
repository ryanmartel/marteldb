use source_index::{location::Location, span::Span};

use crate::{errors::{LexicalError, LexicalErrorKind}, tokens::{TokenKind, TokenValue}};

use self::cursor::Cursor;


mod cursor;

pub struct Lexer<'src> {

    source: &'src str,

    // Current index into the source string.
    cursor: Cursor<'src>,

    // current token being lexed
    current_token: TokenKind,

    // current token value
    current_value: TokenValue,

    current_span: Span,

    // errors encountered
    errors: Vec<LexicalError>,
}

impl<'src> Lexer<'src> {

    pub(crate) fn new(source: &'src str) -> Self {
        Lexer {
            source,
            cursor: Cursor::new(source),
            current_token: TokenKind::Unknown,
            current_value: TokenValue::None,
            current_span: Span::new(Location::new(0), Location::new(0)),
            errors: Vec::new()
        }
    }


    pub fn next_token(&mut self) -> TokenKind {
        self.cursor.start_token();
        self.current_value = TokenValue::None;
        self.current_token = self.lex_token();

        self.current_token
    }

    // Skips whitespace, checks that the char is in the ascii set,
    // and returns EndOfFile at end of stream
    fn lex_token(&mut self) -> TokenKind {
        self.skip_whitespace();
        // may have skipped whitespace, so move the cursor start
        self.cursor.start_token();

        if let Some(c) = self.cursor.bump() {
            // only support ascii text
            if c.is_ascii() {
                self.lex_ascii(c)
            } else {
                // TokenKind::Unknown
                self.push_error(LexicalError::new(
                        LexicalErrorKind::InvalidToken, self.token_range()))
            }
        } else {
            // reached end of file
            TokenKind::EndOfFIle
        }
        
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.cursor.first() {
                ' ' | '\n' | '\t' => self.cursor.bump(),
                _ => break,
            };
        }
    }

    // Push error onto error list and return TokenKind::Unknown
    fn push_error(&mut self, error: LexicalError) -> TokenKind {
        self.errors.push(error);
        TokenKind::Unknown
    }

    fn lex_ascii(&mut self, c: char) -> TokenKind {
        let token = match c {
            c if is_identifier_start(c) => self.lex_identifier_or_keyword(c),
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            ';' => TokenKind::Semicolon,
            _ => self.push_error(LexicalError::new(
                    LexicalErrorKind::InvalidToken, self.token_range()
            ))
        };
        token
    }

    fn lex_identifier_or_keyword(&mut self, c: char) -> TokenKind {
        self.cursor.eat_while(|c| is_identifier_rest(c));
        let text = self.token_text();
        match text.to_uppercase().as_str() {
            "SELECT" => TokenKind::Select,
            _ => {
                TokenKind::Identifier
            }
        }
    }

    fn token_text(&self) -> &'src str {
        &self.source[self.token_range()]
    }

    fn token_range(&self) -> Span {
        let end = self.offset();
        let len = self.cursor.token_length();

        Span::new(end - len, len)
    }

    fn offset(&self) -> Location {
        Location::new(self.source.len()) - self.cursor.text_len()
    }
}

fn is_identifier_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_identifier_rest(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '-')
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn multiple_single_char_tokens() {
        let source = "()";
        let mut lexer = Lexer::new(source);
        assert!(matches!(lexer.next_token(), TokenKind::LParen));
        assert!(matches!(lexer.next_token(), TokenKind::RParen));
    }

    #[test]
    fn skips_whitespace() {
        let source = "() ;  ;
        ;";
        let mut lexer = Lexer::new(source);
        assert!(matches!(lexer.next_token(), TokenKind::LParen));
        assert!(matches!(lexer.next_token(), TokenKind::RParen));
        assert!(matches!(lexer.next_token(), TokenKind::Semicolon));
        assert!(matches!(lexer.next_token(), TokenKind::Semicolon));
        assert!(matches!(lexer.next_token(), TokenKind::Semicolon));
    }

    #[test]
    fn simple_keyword() {
        let source = "SELECT";
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Select),
            "Did not get SELECT token. received {token}");
    }

    #[test]
    fn lowercase_keyword() {
        let source = "select";
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Select),
            "Did not get Select token. received {token}");
    }

    #[test]
    fn simple_identifier() {
        let source = "col";
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Identifier),
            "Did not get Identifier token. received {token}");
    }


}
