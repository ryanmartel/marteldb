use parsing_error::LexicalError;

use crate::{cursor::Cursor, token::{TokenValue, TokenKind}};

#[derive(Debug)]
pub struct Lexer<'src> {
    // source code to be lexed
    source: &'src str,

    // pointer to the current character of the source code which is being lexed.
    cursor: Cursor<'src>,

    // Range of current token in source
    current_range: (u32, u32),

    // Kind of the current token
    current_kind: TokenKind,

    // Value of the current token
    current_value: TokenValue,

    // Errors encountered while lexing
    errors: Vec<LexicalError>,
}

impl<'src> Lexer<'src> {

    pub fn new(source: &'src str) -> Self {
        assert!(u32::try_from(source.len()).is_ok(), "Lexer only supports up to 4GB files");
        Lexer {
            source,
            cursor: Cursor::new(source),
            current_kind: TokenKind::EndOfFile,
            current_value: TokenValue::None,
            current_range: (0, 0),
            errors: Vec::new(),
        }
    }

    pub fn current_kind(&self) -> TokenKind {
        self.current_kind
    }

    pub fn current_range(&self) -> (u32, u32) {
        self.current_range
    }

    pub fn take_value(&mut self) -> TokenValue {
        std::mem::take(&mut self.current_value)
    }

    // Bump the token source to the next token.
    pub fn  bump(&mut self, kind: TokenKind) {
        loop {
            let kind = self.next_token();
        }
    }

    // Helper function to push the given error. Updates current range with the error location
    // and returns TokenKind::Unknown token.
    fn push_error(&mut self, error: LexicalError) -> TokenKind {
        self.current_range = error.location();
        self.errors.push(error);
        TokenKind::Unknown
    }

    // Lex the next token
    fn next_token(&mut self) -> TokenKind {
        self.cursor.start_token();
        self.current_value = TokenValue::None;
        self.current_kind = self.lex_token();
        // For `Unknown` token, the `push_error` method updates the current range
        if !matches!(self.current_kind, TokenKind::Unknown) {
            self.current_range = self.token_range();
        }
        self.current_kind
    }

    fn lex_token(&mut self) -> TokenKind {
        if let Err(error) = self.skip_whitespace() {
            return self.push_error(error);
        }
        // Might have skipped whitespace
        self.cursor.start_token();

        if let Some(c) = self.cursor.bump() {
            if c.is_ascii() {
                print!("{}",c);
            }
        }
        TokenKind::Unknown
    }

    fn offset(&self) -> u32 {
        (self.source.len() as u32) - self.cursor.text_len()
    }

    fn token_range(&self) -> (u32, u32) {
        let end = self.offset();
        let len = self.cursor.token_len();
        (end - len, len)
    }

    fn skip_whitespace(&mut self) -> Result<(), LexicalError> {
        loop {
            match self.cursor.first() {
                ' ' => {
                    self.cursor.bump();
                }
                '\t' => {
                    self.cursor.bump();
                }
                '\n' => {
                    self.cursor.bump();
                }
                _ => break,
            }
        }
        Ok(())
    }


    // Create a checkpoint to which the lexer can return using Self::rewind
    fn checkpoint(&self) -> LexerCheckpoint {
        LexerCheckpoint {
            value: self.current_value.clone(),
            current_kind: self.current_kind,
            current_range: self.current_range,
            cursor_offset: self.offset(),
            errors_position: self.errors.len(),
        }
    }

    // Restore lexer to given checkpoint.
    fn rewind(&mut self, checkpoint: LexerCheckpoint) {
        let LexerCheckpoint {
            value,
            current_kind,
            current_range,
            cursor_offset,
            errors_position,
        } = checkpoint;

        let mut cursor = Cursor::new(self.source);
        // Preserve previous char
        cursor.skip_bytes(cursor_offset);

        self.current_value = value;
        self.current_kind = current_kind;
        self.current_range = current_range;
        self.cursor = cursor;
        self.errors.truncate(errors_position);
    }

    pub fn finish(self) -> Vec<LexicalError> {
        self.errors
    }

}

const fn is_ascii_identifier_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

pub struct LexerCheckpoint {
    value: TokenValue,
    current_kind: TokenKind,
    current_range: (u32, u32),
    cursor_offset: u32,
    errors_position: usize,
}

