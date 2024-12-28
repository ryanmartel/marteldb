use ast::prettyprinter::PrettyPrinter;
use ast::visitor::Visitor;
use parser::parse_stmts;
use parser::parser::Parser;

fn main() {
    let line = "BEGIN;
SAVEPOINT s1;
COMMIT;";

    let mut printer = PrettyPrinter::new();
    let result = parse_stmts(line);
    let parsed = result.unwrap();
    for stmt in parsed.stmts.body {
        printer.visit_stmt(&stmt);
    }
}
