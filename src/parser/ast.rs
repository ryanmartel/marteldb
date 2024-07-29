
#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub begin: usize,
    pub end: usize,
    pub kind: StmtKind,
}

impl Stmt {

}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    // SELECT statement
    Select(Box<SelectStmt>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct SelectStmt {
    pub distinct: bool,
    pub results: Vec<ResultCol>,
    // pub from: Option<FromClause>,
    // pub filter: Option<WhereClause>,
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
    Col(Option<String>, String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    
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
    pub name: String,
}










