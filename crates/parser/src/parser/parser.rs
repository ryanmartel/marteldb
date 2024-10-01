use crate::token_source::TokenSource;

pub struct Parser<'src> {
    // source string
    source: &'src str,

    tokens: TokenSource<'src>,

}
