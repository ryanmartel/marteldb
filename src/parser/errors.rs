use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};

// Define error types here
pub enum Error {
    // Phase out, once specific errors are fleshed out
    ParseError(Item)
}

impl Error {

    pub fn report(&self) -> Diagnostic<()> {
        match self {
            Error::ParseError(stmt) => Diagnostic::error()
                .with_code("E0001")
                .with_message("General Parsing Error")
                .with_labels(vec![
                    Label::primary((), stmt.range.clone()).with_message("Parsing Error")
                ])
        }
    }
}

// Item in the source to be used in the Error enum
pub struct Item {
    range: Range<usize>,
    content: String,
}

impl Item {

    pub fn new(range: Range<usize>, content: impl Into<String>) -> Self {
        let content = content.into();
        Item { range, content }
    }
}
