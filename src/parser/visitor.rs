use super::ast;

pub trait Visitor: Sized {


    fn visit_stmt(&mut self, stmt: &ast::Stmt) {
        walk_stmt(self, stmt);
    }

    fn visit_select_stmt(&mut self, select_stmt: &ast::SelectStmt) {
        walk_select_stmt(self, select_stmt);
    }

    fn visit_insert_stmt(&mut self, insert_stmt: &ast::InsertStmt) {
        walk_insert_stmt(self, insert_stmt);
    }

    fn visit_result_col(&mut self, result_col: &ast::ResultCol) {
        walk_result_col(self, result_col);
    }

    fn visit_from_table(&mut self, from_table: &ast::FromTable) {
        walk_from_table(self, from_table);
    }

    fn visit_where_clause(&mut self, where_clause: &ast::WhereClause) {
        walk_where_clause(self, where_clause);
    }

    fn visit_literal_value(&mut self, literal_value: &ast::LiteralValue) {
        walk_literal_value(self, literal_value);
    }

}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &ast::Stmt) {
    match stmt.kind {
        ast::StmtKind::Select(ref select_stmt) => {visitor.visit_select_stmt(select_stmt)}
        ast::StmtKind::Insert(ref insert_stmt) => {visitor.visit_insert_stmt(insert_stmt)}
    }
} 

pub fn walk_select_stmt<V: Visitor>(visitor: &mut V, select_stmt: &ast::SelectStmt) {
    for i in &select_stmt.results {
        visitor.visit_result_col(i);
    }
    for i in &select_stmt.from {
        visitor.visit_from_table(i);
    }
    match &select_stmt.filter {
        None => {}
        Some(ref where_clause) => { visitor.visit_where_clause(where_clause)}
    }
}

pub fn walk_insert_stmt<V: Visitor>(visitor: &mut V, insert_stmt: &ast::InsertStmt) {
    match insert_stmt.kind {
        ast::InsertStmtKind::Single(ref literal_values) => {
            for i in literal_values {
                visitor.visit_literal_value(i);
            }
        }
        ast::InsertStmtKind::Bulk(ref select_stmt) => {visitor.visit_select_stmt(select_stmt)}
    }
}

pub fn walk_result_col<V: Visitor>(visitor: &mut V, result_col: &ast::ResultCol) {

}

pub fn walk_from_table<V: Visitor>(visitor: &mut V, from_table: &ast::FromTable) {

}

pub fn walk_where_clause<V: Visitor>(visitor: &mut V, where_clause: &ast::WhereClause) {

}

pub fn walk_literal_value<V: Visitor>(visitor: &mut V, literal_value: &ast::LiteralValue) {

}
