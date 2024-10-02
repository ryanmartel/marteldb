use source_index::span::Span;

use crate::name::Name;

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
