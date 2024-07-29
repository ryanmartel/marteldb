use super::visitor::Visitor;

pub struct PrettyPrinter;

impl Visitor for PrettyPrinter {
    
    fn visit_stmt(&mut self, stmt: &super::ast::Stmt) {

    }

    fn visit_select_stmt(&mut self, select_stmt: &super::ast::SelectStmt) {
        
    }
}
