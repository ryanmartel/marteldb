use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Clone, Debug)]
pub struct Cursor<'src> {
    // iterator over the char's of source code
    chars: Chars<'src>,

    // Length of source code. This is used as a marker to indicate
    // the start of the current token being lexed
    source_length: u32,
}

impl<'src> Cursor<'src> {
    pub(crate) fn new(source: &'src str) -> Self {
        Self {
            source_length: (source.len() as u32),
            chars: source.chars()
        }
    }

    // Peek at next character from input stream without consuming it.
    // Return EOF_CHAR if past end of file.
    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    // Peek at second character from input stream without consumption.
    // Return EOF_CHAR if past the end of file.
    pub fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    // Return remaining text to lex
    pub fn rest(&self) -> &'src str {
        self.chars.as_str()
    }

    // Return length of remaining text
    //
    // #Panic
    // Will panic if length remaining is > u32 size. ~4Gb
    // An Sql file really shouldn't be that big.
    pub fn text_len(&self) -> u32 {
        self.chars.as_str().len() as u32
    }

    // Return the length of the current token.
    // To be used after setting the start position of the token
    // using start_token
    pub fn token_len(&self) -> u32 {
        self.source_length - self.text_len()
    }

    // Mark the current position as the start of the token to be lexed
    pub fn start_token(&mut self) {
        self.source_length = self.text_len();
    }

    // Return true if cursor at end of file.
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    // Move the cursor to the next character, returning the previous character
    // Return None if there is no next character
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    // If the next character matches the given character 
    // advance the cursor and return true. Otherwise
    // return false
    pub fn eat_char(&mut self, c: char) -> bool {
        if self.first() == c {
            self.bump();
            true
        } else {
            false
        }
    }

}
