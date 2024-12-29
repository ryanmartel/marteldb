use ast::prettyprinter::PrettyPrinter;
use ast::visitor::Visitor;
use parser::parse_stmts;

fn main() {
    let line = "BEGIN;
SAVEPOINT s1;
COMMIT;
ROLLBACK TRANSACTION;
RELEASE s1;
ROLLBACK TRANSACTION TO SAVEPOINT s1;";

    let mut printer = PrettyPrinter::new();
    let result = parse_stmts(line);
    let parsed = result.unwrap();
    for stmt in parsed.stmts.body {
        printer.visit_stmt(&stmt);
    }
}
