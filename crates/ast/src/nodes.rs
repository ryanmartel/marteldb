use source_index::span::{Span, Spanned};

use crate::name::Name;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Begin(StmtBegin),
    Commit(StmtCommit),
    Invalid(StmtInvalid),
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
pub struct StmtCommit {
    pub span: Span,
}

impl From<StmtCommit> for Stmt {
    fn from(value: StmtCommit) -> Self {
        Stmt::Commit(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtInvalid {
    pub span: Span,
}

impl From<StmtInvalid> for Stmt {
    fn from(value: StmtInvalid) -> Self {
        Stmt::Invalid(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // BinOp(ExprBinOp),
    // UnaryOp(ExprUnaryOp),
    StringLiteral(ExprStringLiteral),
    BooleanLiteral(ExprBooleanLiteral),
    NullLiteral(ExprNullLiteral),
    IntLiteral(ExprIntLiteral),
    FloatLiteral(ExprFloatLiteral),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStringLiteral {
    pub span: Span,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBooleanLiteral {
    pub span: Span,
    pub value: bool

}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprNullLiteral {
    pub span: Span,

}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprIntLiteral {
    pub span: Span,
    pub value: i64,

}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprFloatLiteral {
    pub span: Span,
    pub value: f64

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

