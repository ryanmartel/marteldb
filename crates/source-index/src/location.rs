use std::{fmt::Display, ops::Sub, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Location(usize);

impl std::ops::Deref for Location {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Location {
    pub fn new(l: usize) -> Self {
        Self::from(l)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let loc = self.0;
        write!(f, "{loc}")
    }
}

impl From<usize> for Location {
    fn from(value: usize) -> Self {
        Location(value)
    }
}

impl From<Location> for usize {
    fn from(value: Location) -> Self {
        value.0
    }
}

impl Sub for Location {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Add for Location {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}
