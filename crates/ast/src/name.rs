use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }

    pub fn is_valid(&self) -> bool {
        if self.0 == "" {
            return false;
        }
        return true;
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.0)
    }
}
