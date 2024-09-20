// use super::visitor::*;
// use super::ast;
// use super::parsing_errors::*;
// use std::mem;
//
// pub struct PrettyPrinter {
//     spaces: u32,
//     errors: Vec<Error>
// }
//
// impl Visitor for PrettyPrinter {
//
//     fn errors(&mut self) -> Vec<Error> {
//         return mem::take(&mut self.errors);
//     }
//     
//     fn visit_stmt(&mut self, stmt: &ast::Stmt) {
//         self.print_space();
//         println!("Statement:");
//         self.indent();
//         walk_stmt(self, stmt);
//         self.dedent();
//     }
//
//     fn visit_create_table(&mut self, create_table: &ast::CreateTable) {
//         self.print_space();
//         println!("Create Table: ");
//         self.indent();
//         self.print_space();
//         println!("Table name - {}",create_table.table );
//         self.dedent();
//     }
//
//     fn visit_select_stmt(&mut self, select_stmt: &ast::SelectStmt) {
//         self.print_space();
//         println!("Select Stmt:");
//         self.indent();
//         self.print_space();
//         println!("distinct? - {}", select_stmt.distinct);
//         walk_select_stmt(self, select_stmt);
//         self.dedent();
//     }
//
//     fn visit_insert_stmt(&mut self, insert_stmt: &ast::InsertStmt) {
//         self.print_space();
//         println!("Insert Stmt:");
//         self.indent();
//         self.print_space();
//         println!("INTO table - {}", insert_stmt.table);
//         match insert_stmt.cols {
//             None => {}
//             Some(ref cols) => {
//                 for i in cols {
//                     self.print_space();
//                     println!("COL - {}", i);
//                 }
//             }
//         }
//         walk_insert_stmt(self, insert_stmt);
//         self.dedent();
//     }
//
//     fn visit_result_col(&mut self, result_col: &ast::ResultCol) {
//         self.print_space();
//         println!("Result Col:");
//         self.indent();
//         match result_col.kind {
//             ast::ResultColKind::All(ref table) => {
//                 match table {
//                     None => {
//                         self.print_space();
//                         println!("*");
//                     }
//                     Some(tab_str) => {
//                         self.print_space();
//                         println!("{}.*", tab_str);
//                     }
//                 }
//             }
//             _ => {}
//         }
//         walk_result_col(self, result_col);
//         self.dedent();
//     }
//
//     fn visit_from_table(&mut self, from_table: &ast::FromTable) {
//         self.print_space();
//         println!("From Table:");
//         self.indent();
//         self.print_space();
//         match &from_table.kind {
//             ast::FromTableKind::Single(table) => {
//                 if table.name.eq_ignore_ascii_case("tab1") {
//                     self.errors.push(Error::UnknownTable(Item::new(table.begin..table.end, table.name.clone())));
//                 }
//                 println!("table - {}", table);
//             }
//         }
//         walk_from_table(self, from_table);
//         self.dedent();
//     }
//
//     fn visit_where_clause(&mut self, where_clause: &ast::WhereClause) {
//         self.print_space();
//         println!("Where clause:");
//         self.indent();
//         walk_where_clause(self, where_clause);
//         self.dedent();
//     }
//
//     fn visit_literal_value(&mut self, literal_value: &ast::LiteralValue) {
//         self.print_space();
//         println!("literal value:");
//         self.indent();
//         self.print_space();
//         match &literal_value.kind {
//             ast::LiteralValueKind::Null => {
//                 println!("Null");
//             }
//             ast::LiteralValueKind::True => {
//                 println!("true");
//             }
//             ast::LiteralValueKind::False => {
//                 println!("false");
//             }
//             ast::LiteralValueKind::StringLit(str_lit) => {
//                 println!("{}", str_lit);
//             }
//             ast::LiteralValueKind::Numeric(num) => {
//                 println!("{}", num);
//             }
//         }
//         self.dedent();
//     }
//
//     fn visit_table_column(&mut self, table_column: &ast::TableColumn) {
//         self.print_space();
//         println!("Table Column: ");
//         self.indent();
//         self.print_space();
//         match &table_column.table {
//             None => {
//                 println!("{}", table_column.column);
//             }
//             Some(table) => {
//                 println!("{}.{}", table, table_column.column);
//             }
//         }
//         walk_table_column(self, table_column);
//         self.dedent();
//     }
//
//     fn visit_expr(&mut self, expr: &ast::Expr) {
//         self.print_space();
//         println!("Expr: ");
//         self.indent();
//         walk_expr(self, expr);
//         self.dedent();
//     }
//
//     fn visit_binop(&mut self, binop: &ast::BinOp) {
//         self.print_space();
//         println!("BinOp: ");
//         self.indent();
//         walk_binop(self, binop);
//         self.dedent();
//     }
//
//     fn visit_unop(&mut self, unop: &ast::UnOp) {
//         self.print_space();
//         println!("UnOp: ");
//         self.indent();
//         walk_unop(self, unop);
//         self.dedent();
//     }
// }
//
// impl PrettyPrinter {
//     pub fn new() -> Self {
//         Self {
//             spaces: 0,
//             errors: Vec::new()
//         }
//     }
//
//     fn print_space(&self) {
//        let mut index = 0; 
//        while index < self.spaces {
//            print!("  ");
//            index += 1;
//        }
//
//     }
//     fn indent(&mut self) {
//         self.spaces += 1;
//     }
//
//     fn dedent(&mut self) {
//         self.spaces -= 1;
//     }
// }
