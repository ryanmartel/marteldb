use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Script {
    Script(StmtList),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtList {
    pub range: (usize, usize),
    pub body: Vec<Stmt>,
}

impl From<StmtList> for Script {
    fn from(payload: StmtList) -> Self {
        Script::Script(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    AlterTable(AlterTableStmt),
    Begin(BeginStmt),
    Commit(CommitStmt),
    CreateIndex(CreateIndexStmt),
    CreateTable(CreateTableStmt),
    Delete(DeleteStmt),
    DropIndex(DropIndexStmt),
    DropTable(DropTableStmt),
    Insert(InsertStmt),
    Reindex(ReindexStmt),
    Release(ReleaseStmt),
    Rollback(RollbackStmt),
    Savepoint(SavepointStmt),
    Select(StmtSelect),
    Update(UpdateStmt),
    Vacuum(VacuumStmt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterTableStmt {

}

impl From<AlterTableStmt> for Stmt {
    fn from(payload: AlterTableStmt) -> Self {
        Stmt::AlterTable(payload)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct BeginStmt {

}

impl From<BeginStmt> for Stmt {
    fn from(payload: BeginStmt) -> Self {
        Stmt::Begin(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommitStmt {

}

impl From<CommitStmt> for Stmt {
    fn from(payload: CommitStmt) -> Self {
        Stmt::Commit(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateIndexStmt {

}

impl From<CreateIndexStmt> for Stmt {
    fn from(payload: CreateIndexStmt) -> Self {
        Stmt::CreateIndex(payload)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct CreateTableStmt {

}

impl From<CreateTableStmt> for Stmt {
    fn from(payload: CreateTableStmt) -> Self {
        Stmt::CreateTable(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteStmt {

}

impl From<DeleteStmt> for Stmt {
    fn from(payload: DeleteStmt) -> Self {
        Stmt::Delete(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DropIndexStmt {

}

impl From<DropIndexStmt> for Stmt {
    fn from(payload: DropIndexStmt) -> Self {
        Stmt::DropIndex(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DropTableStmt {

}

impl From<DropTableStmt> for Stmt {
    fn from(payload: DropTableStmt) -> Self {
        Stmt::DropTable(payload)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct InsertStmt {

}

impl From<InsertStmt> for Stmt {
    fn from(payload: InsertStmt) -> Self {
        Stmt::Insert(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReindexStmt {

}

impl From<ReindexStmt> for Stmt {
    fn from(payload: ReindexStmt) -> Self {
        Stmt::Reindex(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReleaseStmt {

}

impl From<ReleaseStmt> for Stmt {
    fn from(payload: ReleaseStmt) -> Self {
        Stmt::Release(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RollbackStmt {

}

impl From<RollbackStmt> for Stmt {
    fn from(payload: RollbackStmt) -> Self {
        Stmt::Rollback(payload)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct SavepointStmt {

}

impl From<SavepointStmt> for Stmt {
    fn from(payload: SavepointStmt) -> Self {
        Stmt::Savepoint(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtSelect {

}

impl From<StmtSelect> for Stmt {
    fn from(payload: StmtSelect) -> Self {
        Stmt::Select(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStmt {

}

impl From<UpdateStmt> for Stmt {
    fn from(payload: UpdateStmt) -> Self {
        Stmt::Update(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VacuumStmt {

}

impl From<VacuumStmt> for Stmt {
    fn from(payload: VacuumStmt) -> Self {
        Stmt::Vacuum(payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    BoolOp
}


#[derive(Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub enum BoolOp {

}

#[derive(Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    BitOr,
    BitAnd,
    LShift,
    RShift,
}

impl Operator {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mult => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::BitOr => "|",
            Operator::BitAnd => "&",
            Operator::LShift => "<<",
            Operator::RShift => ">>",
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub enum UnaryOp {

}

#[derive(Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub enum CmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    IsDistinctFrom,
    IsNotDistinctFrom,
    Like,
    Match,
    NotLike,
    NotMatch,
}

impl CmpOp {
    pub const fn as_str(&self) -> &'static str {
        match self {
            CmpOp::Eq => "==",
            CmpOp::NotEq => "!=",
            CmpOp::Lt => "<",
            CmpOp::LtE => "<=",
            CmpOp::Gt => ">",
            CmpOp::GtE => ">=",
            CmpOp::Is => "IS",
            CmpOp::IsNot => "IS NOT",
            CmpOp::IsDistinctFrom => "IS DISTINCT FROM",
            CmpOp::IsNotDistinctFrom => "IS NOT DISTINCT FROM",
            CmpOp::Like => "LIKE",
            CmpOp::Match => "MATCH",
            CmpOp::NotLike => "NOT LIKE",
            CmpOp::NotMatch => "NOT MATCH",
        }
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

