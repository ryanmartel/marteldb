use crate::{self as ast, Stmt};

pub trait Visitor {
    fn visit_stmt(&mut self, stmt: &Stmt) {

    }
}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &Stmt) {
    match stmt {
        Stmt::Begin(ast::StmtBegin{..}) => {

        }
        Stmt::Commit(ast::StmtCommit{..}) => {

        }
        Stmt::Invalid(ast::StmtInvalid{..}) => {

        }
        Stmt::Savepoint(ast::StmtSavepoint{..}) => {
            
        }
    }
}
