#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Variable {
        name: String,
    }
}
