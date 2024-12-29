use source_index::span::{Span, Spanned};

use crate::name::Name;


#[derive(Clone, Debug, PartialEq)]
pub struct Stmts {
    pub span: Span,
    pub body: Vec<Stmt>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Begin(StmtBegin),
    Commit(StmtCommit),
    Invalid(StmtInvalid),
    Release(StmtRelease),
    Rollback(StmtRollback),
    Savepoint(StmtSavepoint),
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
pub struct StmtRelease {
    pub span: Span,
    pub id: Identifier,
}

impl From<StmtRelease> for Stmt {
    fn from(value: StmtRelease) -> Self {
        Stmt::Release(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtRollback {
    pub span: Span,
    pub id: Option<Identifier>,
}

impl From<StmtRollback> for Stmt {
    fn from(value: StmtRollback) -> Self {
        Stmt::Rollback(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtSavepoint {
    pub span: Span,
    pub id: Identifier,
}

impl From<StmtSavepoint> for Stmt {
    fn from(value: StmtSavepoint) -> Self {
        Stmt::Savepoint(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    BinOp(ExprBinaryOp),
    UnaryOp(ExprUnaryOp),
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
pub struct ExprBinaryOp {
    pub span: Span,
    pub left: Box<Expr>,
    pub op: BinaryOperator,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprUnaryOp {
    pub span: Span,
    pub op: UnaryOperator,
    pub operand: Box<Expr>,
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

#[derive(Clone, Debug, PartialEq)]
pub enum BooleanOperator {
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CmpOperator {
    Gt,
    GtE,
    Lt,
    LtE,
    NotEq,
    Eq
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Positive,
    Negative,
}

