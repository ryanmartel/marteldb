use ast::prettyprinter::PrettyPrinter;
use ast::visitor::Visitor;
use parser::parse_stmts;

fn main() {
    let line = "BEGIN;
SAVEPOINT s1;
COMMIT;
ROLLBACK TRANSACTION;
RELEASE s1;
ROLLBACK TRANSACTION TO SAVEPOINT s1;
DROP TABLE IF EXISTS t1;";

    let mut printer = PrettyPrinter::new();
    let result = parse_stmts(line);
    match result {
        Ok(parsed) => {
            for stmt in parsed.stmts.body {
                printer.visit_stmt(&stmt);
            }
        }
        Err(parse_errors) => {
            for err in parse_errors {
                println!("Error: {}\n (span {}, {})", err.kind, err.span.start(), err.span.end());
            }
        }
    }
    // let parsed = result.unwrap();
}
