use super::ast;

pub trait Visitor: Sized {


    fn visit_stmt(&mut self, stmt: &ast::Stmt) {
        walk_stmt(self, stmt);
    }

    fn visit_select_stmt(&mut self, select_stmt: &ast::SelectStmt) {
        walk_select_stmt(self, select_stmt);
    }

}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &ast::Stmt) {

} 

pub fn walk_select_stmt<V: Visitor>(visitor: &mut V, select_stmt: &ast::SelectStmt) {

}

