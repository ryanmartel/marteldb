use std::ops::{Index, Range};

use crate::location::Location;

#[derive(Debug, Copy, Clone)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {

    pub fn new(start: Location, end: Location) -> Self {
        Self {
            start,
            end,
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
    T: From<Location>
{
    fn from(value: Span) -> Self {
        value.start.into()..value.end.into()
    }
}
