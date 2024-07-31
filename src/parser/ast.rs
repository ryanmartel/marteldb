
#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub begin: usize,
    pub end: usize,
    pub kind: StmtKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    // SELECT statement
    Select(Box<SelectStmt>),
    // INSERT
    Insert(Box<InsertStmt>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertStmt {
    pub table: String,
    pub cols: Option<Vec<String>>,
    pub values: Vec<LiteralValue>
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectStmt {
    pub distinct: bool,
    pub results: Vec<ResultCol>,
    pub from: Vec<FromTable>,
    pub filter: Option<WhereClause>,
    // pub group_by: Option<GroupByClause>,
    // pub except: Option<Box<SelectStmt>>,
    // pub order_by: Option<OrderByClause>,
    // pub limit: Option<LimitClause>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResultCol {
    pub kind: ResultColKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResultColKind {
    All(Option<String>),
    TableColumn(TableColumn)
}

#[derive(Clone, Debug, PartialEq)]
pub struct FromTable {
    pub kind: FromTableKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FromTableKind {
    Single(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {
    pub expr: Expr
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {

    SelectStmt(Box<SelectStmt>),
    Literal(LiteralValue),
    Column(TableColumn),
    // Tri
    Between(Box<Expr>, Box<Expr>, Box<Expr>),
    // Binop
    Binop(BinOp, Box<Expr>, Box<Expr>),
    // Unop
    Unop(UnOp, Box<Expr>)
    
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableColumn {
    pub table: Option<String>,
    pub column: String,
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
    // SQL specific
    Is,
    In,

    // Logical
    And,
    Or,

    // Mathematical
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
    Not,
    Exists
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: String,
}










