#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
}

impl Stmt {

}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    Variable {
        name: String,
    },
}
