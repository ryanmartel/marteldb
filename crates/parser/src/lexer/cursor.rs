use std::str::Chars;

use source_index::location::Location;

pub const EOF_CHAR: char = '\0';

/// A Cursor that marks the current lexer location.
pub(super) struct Cursor<'src> {

    chars: Chars<'src>,

    source_length: Location,
    
    current: Location,
}

impl<'src> Cursor<'src> {

    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
            source_length: Location::new(source.len()),
            current: Location::new(0),
        }
    }

    /// Peek at the next character of input stream without conumsing it.
    /// Returns `[EOF_CHAR]` if position is past end of file.
    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Return length of remaining text
    pub fn text_len(&self) -> Location {
        Location::new(self.chars.as_str().len())
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    //? The current token length.
    // used after `[start_token]` is used.
    pub fn token_length(&self) -> Location {
        self.source_length - self.text_len()
    }

    /// Mark the current position of the cursor as the start position of the next token.
    pub fn start_token(&mut self) {
        self.source_length = self.text_len();
    }

    /// Moves the cursor to the next character, returning the previous character.
    /// returns `[None]` if there is no next character.
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// if the next character matches `c`, move the cursor forward and return true,
    /// otherwise return false
    pub fn eat_char(&mut self, c: char) -> bool {
        if self.first() == c {
            self.bump();
            true
        } else {
            false
        }
    }

    /// consume characters until the predicate is false
    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn eat_if(&mut self, mut predicate: impl FnMut(char) -> bool) -> Option<char> {
        if predicate(self.first()) && !self.is_eof() {
            self.bump()
        } else {
            None
        }
    }

    /// move forward the cursor by `count` bytes
    pub fn seek_forward(&mut self, count: usize) {
        self.chars = self.chars.as_str()[count..].chars();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_chars() {
        let source = "ABC";

        let mut cursor = Cursor::new(source);
        assert!(cursor.eat_char('A'));
        assert!(!cursor.eat_char('b'));
        assert!(cursor.eat_char('B'));
        assert!(cursor.eat_char('C'));
    }

    #[test]
    fn seek_and_eat() {
        let source = "ABCDEF90-809(008dfascv)";

        let mut cursor = Cursor::new(source);
        cursor.seek_forward(10);
        assert!(cursor.eat_char('0'));
    }
}


