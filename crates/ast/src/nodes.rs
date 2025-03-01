use std::fmt::Display;

use source_index::span::{Span, Spanned};

use crate::name::Name;

#[derive(Clone, Debug, PartialEq)]
pub struct Stmts {
    pub span: Span,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Alter(StmtAlter),
    Begin(StmtBegin),
    Commit(StmtCommit),
    Drop(StmtDrop),
    Invalid(StmtInvalid),
    Reindex(StmtReindex),
    Release(StmtRelease),
    Rollback(StmtRollback),
    Savepoint(StmtSavepoint),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtAlter {
    pub span: Span,
    pub id: Identifier,
    pub action: AlterTableAction
}

impl From<StmtAlter> for Stmt {
    fn from(value: StmtAlter) -> Self {
        Stmt::Alter(value)
    }
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
pub struct StmtDrop {
    pub span: Span,
    pub kind: DdlTargetKind,
    pub exist_check: bool,
    pub id: Identifier,

}

/// DDL target for CREATE and DROP statements
#[derive(Clone, Debug, PartialEq)]
pub enum DdlTargetKind {
    Table,
    Index,
}

impl Display for DdlTargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Table => write!(f, "TABLE"),
            Self::Index => write!(f, "INDEX"),
        }
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
pub struct StmtReindex {
    pub span: Span,
    pub id: Option<Identifier>
}

impl From<StmtReindex> for Stmt {
    fn from(value: StmtReindex) -> Self {
        Stmt::Reindex(value)
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
pub struct AlterTableAction {
    pub span: Span,
    pub kind: AlterTableActionKind
}

#[derive(Clone, Debug, PartialEq)]
pub enum AlterTableActionKind {
    Rename(AlterTableRename),
    Add(AlterTableAdd),
    Drop(AlterTableDrop)
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterTableRename {
    pub span: Span,
    pub kind: AlterTableRenameKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AlterTableRenameKind {
    Table(Identifier),
    Column(Identifier, Identifier)
}


#[derive(Clone, Debug, PartialEq)]
pub struct AlterTableAdd {
    pub span: Span,
    pub column: ColumnDef
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterTableDrop {
    pub span: Span,
    pub id: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnDef {
    pub span: Span,
    pub id: Identifier,
    pub type_name: TypeName,
    pub constraint_list: ColumnConstraintList
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnConstraintList {
    pub span: Span,
    pub constraints: Vec<ColumnConstraint>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnConstraint {
    pub span: Span,
    pub kind: ColumnConstraintKind
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnConstraintKind {
    PrimaryKey(ColumnConstraintPrimaryKey),
    NotNull(Option<ConflictAction>),
    Unique(Option<ConflictAction>),
    Check(Expr),
    Default(ColumnConstraintDefault),
    Collate(Identifier),
    Foreign(ForeignKeyClause),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnConstraintPrimaryKey {
    pub span: Span,
    pub order: Option<Order>,
    pub conflict_action: Option<ConflictAction>

}

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnConstraintDefault {
    ParenExpr(Expr),
    LiteralValue(LiteralValue)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForeignKeyClause {
    pub span: Span,
    pub id: Identifier,
    pub column_names: Vec<Identifier>,
    pub foreign_key_clause_on: Option<ForeignKeyClauseOn>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForeignKeyClauseOn {
    pub span: Span,
    pub kind: ForeignKeyClauseOnKind,
    pub action: ForeignKeyClauseActions,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForeignKeyClauseOnKind {
    Delete,
    Update
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForeignKeyClauseActions {
    Set(ForeignKeyClauseActionSet),
    Cascade,
    Restrict,
    NoAction
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForeignKeyClauseActionSet {
    Null,
    Default
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConflictAction {
    Rollback,
    Abort,
    Fail,
    Ignore,
    Replace
}


#[derive(Clone, Debug, PartialEq)]
pub enum Order {
    Asc,
    Desc
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeName {
    pub span: Span,
    pub external_type: ExternalType,
    pub number_field: Option<TypeNameNumberField>
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalType {
    Char,
    Integer,
    Numeric,
    Serial,
    Varchar
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeNameNumberField {
    pub span: Span,
    pub first: SignedNumber,
    pub second: Option<SignedNumber>
}


#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    BinOp(ExprBinaryOp),
    UnaryOp(ExprUnaryOp),
    LiteralValue(LiteralValue),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SignedNumber {
    Integer(IntLiteral),
    Float(FloatLiteral),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    StringLiteral(StringLiteral),
    IntLiteral(IntLiteral),
    FloatLiteral(FloatLiteral),
    NullLiteral(NullLiteral),
    BoolLiteral(BoolLiteral),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringLiteral {
    pub span: Span,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoolLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NullLiteral {
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntLiteral {
    pub span: Span,
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FloatLiteral {
    pub span: Span,
    pub value: f64,
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
    Eq,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Positive,
    Negative,
}
