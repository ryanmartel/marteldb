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
pub struct Expr {
    pub kind: ExprKind,
}

impl Expr {

}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Lit(LiteralValue),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Column(Option<Ident>, Ident),
    ExprList(Vec<Expr>),
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










