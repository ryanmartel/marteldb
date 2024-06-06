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
    // CREATE TABLE
    CreateTable(Box<CreateTableStmt>),
    // DROP TABLE
    DropTable(Box<DropTableStmt>),
    // SELECT statement
    Select(Box<SelectStmt>),
    // DELETE statement
    Delete(Box<DeleteStmt>),
    // INSERT statement,
    Insert(Box<InsertStmt>),
    // UPDATE statement,
    Update(Box<UpdateStmt>),

}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateTableStmt {

}

#[derive(Clone, Debug, PartialEq)]
pub struct DropTableStmt {

}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectStmt {
    pub distinct: bool,
    pub results: Vec<ResultCol>,
    pub from: Option<FromClause>,
    pub filter: Option<WhereClause>,
    pub group_by: Option<GroupByClause>,
    pub except: Option<Box<SelectStmt>>,
    pub order_by: Option<OrderByClause>,
    pub limit: Option<LimitClause>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FromClause {
    pub kind: FromClauseKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FromClauseKind {
    Join
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteStmt {

}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertStmt {

}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStmt {

}

#[derive(Clone, Debug, PartialEq)]
pub struct ResultCol {
    pub kind: ResultColKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResultColKind {
    // * || tableName.*
    All(Option<Ident>),
    // expr >> (AS colum-alias)?
    ExprRes(Box<Expr>, Option<Ident>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

impl Expr {

}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    // Literal Value
    Lit(LiteralValue),
    // expr >> BinOp >> expr
    Binary(BinOp, Box<Expr>, Box<Expr>),
    // UnOp >> expr
    Unary(UnOp, Box<Expr>),
    // tableName >> columnName || columnName
    Column(Option<Ident>, Ident),
    // Expr, ...
    ExprList(Vec<Expr>),
    // expr >> IS NOT? >> expr
    ExprIS(Box<Expr>, NOTVal, Box<Expr>),
    // expr >> NOT? BETWEEN >> expr >> AND expr
    ExprBetween(Box<Expr>, NOTVal, Box<Expr>, Box<Expr>),
    // expr >> NOT? IN >> InExpr
    ExprIN(Box<Expr>, NOTVal, Box<InExpr>),
    // NOT? Exists? >> Select
    Exists(NOTVal, ExistsVal, Box<SelectStmt>),
    
}

#[derive(Clone, Debug, PartialEq)]
pub struct InExpr {
    pub kind: InExprKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InExprKind {
    Select(Box<SelectStmt>),
    ExprList(Vec<Expr>),
    Table(Ident),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NOTVal {
    pub not: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExistsVal {
    pub exists: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralValue {
    pub kind: LiteralValueKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValueKind {
    Numeric {
        value: i32,
    },
    StringLit {
        value: String,
    },
    Null,
    True,
    False,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinOp {
    pub kind: BinOpKind,
}


#[derive(Clone, Debug, PartialEq)]
pub enum BinOpKind {
    And,
    Or,
    Add,
    Sub,
    Eq,
    Lt,
    LtE,
    Gt,
    GtE,
    Ne,
}

#[derive(Clone, Debug, PartialEq)] 
pub struct UnOp {
    pub kind: UnOpKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnOpKind {
    Plus,
    Minus,
    NotNull,
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    name: String,
}










