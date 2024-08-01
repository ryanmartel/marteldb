use super::visitor::*;
use super::ast;

pub struct PrettyPrinter {
    spaces: u32
}

impl Visitor for PrettyPrinter {
    
    fn visit_stmt(&mut self, stmt: &ast::Stmt) {
        self.print_space();
        println!("Statement:");
        self.indent();
        walk_stmt(self, stmt);
        self.dedent();
    }

    fn visit_select_stmt(&mut self, select_stmt: &ast::SelectStmt) {
        self.print_space();
        println!("Select Stmt:");
        self.indent();
        walk_select_stmt(self, select_stmt);
        self.dedent();
    }

    fn visit_insert_stmt(&mut self, insert_stmt: &ast::InsertStmt) {
        self.print_space();
        println!("Insert Stmt:");
        self.indent();
        self.print_space();
        println!("INTO table - {}", insert_stmt.table);
        match insert_stmt.cols {
            None => {}
            Some(ref cols) => {
                for i in cols {
                    self.print_space();
                    println!("COL - {}", i);
                }
            }
        }
        walk_insert_stmt(self, insert_stmt);
        self.dedent();
    }

    fn visit_result_col(&mut self, result_col: &ast::ResultCol) {
        println!("Result Col:");
        walk_result_col(self, result_col);
    }

    fn visit_from_table(&mut self, from_table: &ast::FromTable) {
        println!("From Table:");
        walk_from_table(self, from_table);
    }

    fn visit_where_clause(&mut self, where_clause: &ast::WhereClause) {
        println!("Where clause:");
        walk_where_clause(self, where_clause);
    }

    fn visit_literal_value(&mut self, literal_value: &ast::LiteralValue) {
        println!("literal value:");
        walk_literal_value(self, literal_value);
    }
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self {
            spaces: 0,
        }
    }

    fn print_space(&self) {
       let mut index = 0; 
       while index < self.spaces {
           print!("  ");
           index += 1;
       }

    }
    fn indent(&mut self) {
        self.spaces += 1;
    }

    fn dedent(&mut self) {
        self.spaces -= 1;
    }
}
