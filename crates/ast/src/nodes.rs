use source_index::span::{Span, Spanned};

use crate::name::Name;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Begin(StmtBegin),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtBegin {
    pub span: Span,
}

impl From<StmtBegin> for Stmt {
    fn from(value: StmtBegin) -> Self {
        Stmt::Begin(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub id: Name,
    pub span: Span,
}

impl Identifier {

    pub fn new(id: impl Into<Name>, span: Span) -> Self {
        Self {
            id: id.into(),
            span,
        }
    }

    pub fn id(&self) -> &Name {
        &self.id
    }

}

impl Spanned for Identifier {
    fn span(&self) -> Span {
        self.span
    }
}


