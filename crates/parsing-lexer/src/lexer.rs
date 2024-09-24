use parsing_error::LexicalError;

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Lexer<'src> {
    // source code to be lexed
    source: &'src str,

    // pointer to the current character of the source code which is being lexed.
    cursor: Cursor<'src>,

    // Errors encountered while lexing
    errors: Vec<LexicalError>,
}

impl<'src> Lexer<'src> {

    pub fn new(source: &'src str) -> Self {
        assert!(u32::try_from(source.len()).is_ok(), "Lexer only supports up to 4GB files");
        Self {
            source,
            cursor: Cursor::new(source),
            errors: Vec::new(),
        }
    }
}
