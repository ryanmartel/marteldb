use std::{fmt::Display, ops::{Index, Range}};

use crate::location::Location;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    // Creates a 0 length span at index
    pub fn empty(offset: Location) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    pub fn start(&self) -> Location {
        self.start
    }

    pub fn end(&self) -> Location {
        self.end
    }
}

impl Index<Span> for str {
    type Output = str;
    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

impl<T> From<Span> for Range<T>
where
    T: From<Location>,
{
    fn from(value: Span) -> Self {
        value.start.into()..value.end.into()
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let loc1 = self.start;
        let loc2 = self.end;
        write!(f, "start: {loc1}, end: {loc2}")
    }
}

// A Spanned item in the source text
pub trait Spanned {
    // The span of this item in the source text
    fn span(&self) -> Span;

    // The start offset of this item
    fn start(&self) -> Location {
        self.span().start()
    }

    // The end offset of this item
    fn end(&self) -> Location {
        self.span().end()
    }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl<T> Spanned for &T
where
    T: Spanned,
{
    fn span(&self) -> Span {
        T::span(self)
    }
}
