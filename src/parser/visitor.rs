use super::ast;

pub trait Visitor: Sized {


    fn visit_stmt(&mut self, stmt: &ast::Stmt) {
        walk_stmt(self, stmt);
    }

    fn visit_create_table(&mut self, create_table: &ast::CreateTable) {

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

    fn visit_from_table(&mut self, _from_table: &ast::FromTable) {
    }

    fn visit_where_clause(&mut self, where_clause: &ast::WhereClause) {
        walk_where_clause(self, where_clause);
    }

    fn visit_literal_value(&mut self, _literal_value: &ast::LiteralValue);

    fn visit_table_column(&mut self, _table_column: &ast::TableColumn);

    fn visit_expr(&mut self, expr: &ast::Expr) {
        walk_expr(self, expr);
    }

    fn visit_binop(&mut self, _binop: &ast::BinOp);

    fn visit_unop(&mut self, _unop: &ast::UnOp);
}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &ast::Stmt) {
    match stmt.kind {
        ast::StmtKind::CreateTable(ref create_table) => {visitor.visit_create_table(create_table)}
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
    match result_col.kind {
        ast::ResultColKind::All(_) => {}
        ast::ResultColKind::TableColumn(ref table_column) => {
            visitor.visit_table_column(table_column)
        }
    }
}

pub fn walk_from_table<V: Visitor>(visitor: &mut V, from_table: &ast::FromTable) {
    match from_table.kind {
        ast::FromTableKind::Single(_) => {}
    }
}

pub fn walk_where_clause<V: Visitor>(visitor: &mut V, where_clause: &ast::WhereClause) {
    visitor.visit_expr(&where_clause.expr)

}

pub fn walk_literal_value<V: Visitor>(visitor: &mut V, literal_value: &ast::LiteralValue) {
}

pub fn walk_table_column<V: Visitor>(visitor: &mut V, table_column: &ast::TableColumn) {

}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &ast::Expr) {
    match expr.kind {
        ast::ExprKind::SelectStmt(ref select_stmt) => {visitor.visit_select_stmt(select_stmt)}
        ast::ExprKind::Literal(ref literal_value) => {visitor.visit_literal_value(literal_value)}
        ast::ExprKind::Column(ref table_column) => {visitor.visit_table_column(table_column)}
        ast::ExprKind::Binop(ref binop, ref e1, ref e2) => {
            visitor.visit_binop(binop);
            visitor.visit_expr(e1);
            visitor.visit_expr(e2);
        }
        ast::ExprKind::Unop(ref unop, ref expr) => {
            visitor.visit_unop(unop);
            visitor.visit_expr(expr);
        }

    }
}

pub fn walk_binop<V: Visitor>(visitor: &mut V, binop: &ast::BinOp) {

}

pub fn walk_unop<V: Visitor>(visitor: &mut V, unop: &ast::UnOp) {

}
