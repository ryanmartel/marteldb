use std::ops::Sub;


#[derive(Debug, Copy, Clone)]
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
