#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
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

