use source_index::location::Location;

use crate::errors::ParseError;

use crate::token_source::TokenSource;


pub struct Parser<'src> {
    // source string
    source: &'src str,

    tokens: TokenSource<'src>,

    errors: Vec<ParseError>,

    prev_token_end: Location,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str) -> Self {
        let tokens = TokenSource::new(source);

        Self {
            source,
            errors: Vec::new(),
            tokens,
            prev_token_end: Location::new(0),
        }
    }
}
