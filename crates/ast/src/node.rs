use crate::{
    self as ast, StmtSelect,
};
pub trait AstNode {
    type Ref<'a>;

    fn cast(kind: AnyNode) -> Option<Self>
        where
            Self: Sized;
    fn cast_ref(kind: AnyNodeRef<'_>) -> Option<Self::Ref<'_>>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnyNode {

}

#[derive(Clone, Debug, PartialEq)]
pub enum AnyNodeRef<'a> {
    StmtSelect(&'a ast::StmtSelect),
}

