use crate::visitor::Visitor;
use crate::{self as ast, Stmt};

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {}
    }
}

impl Visitor for PrettyPrinter {
    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Begin(ast::StmtBegin { span }) => {
                println!("BEGIN (span {}, {})", span.start(), span.end());
            }
            Stmt::Commit(ast::StmtCommit { span }) => {
                println!("COMMIT (span {}, {})", span.start(), span.end());
            }
            Stmt::Invalid(ast::StmtInvalid { span }) => {
                println!("INVALID (span {}, {})", span.start(), span.end());
            }
            Stmt::Release(ast::StmtRelease { span, id }) => {
                println!("RELEASE (span {}, {})", span.start(), span.end());
                println!(
                    "\tid: {} (span {}, {})",
                    &id.id,
                    &id.span.start(),
                    &id.span.end()
                );
            }
            Stmt::Rollback(ast::StmtRollback { span, id }) => {
                println!("ROLLBACK (span {}, {})", span.start(), span.end());
                match id {
                    Some(id) => {
                        println!(
                            "\tid: {} (span {}, {})",
                            &id.id,
                            &id.span.start(),
                            &id.span.end()
                        );
                    }
                    None => {}
                }
            }
            Stmt::Savepoint(ast::StmtSavepoint { span, id }) => {
                println!("SAVEPOINT (span {}, {})", span.start(), span.end());
                println!(
                    "\tid: {} (span {}, {})",
                    &id.id,
                    &id.span.start(),
                    &id.span.end()
                );
            }
            Stmt::Drop(ast::StmtDrop { span, kind, exist_check, id}) => {
                println!("DROP (span {}, {})", span.start(), span.end());
                println!(
                    "\tkind: {}\n\texist check: {}",
                    kind, exist_check
                );
                println!(
                    "\tid: {} (span {}, {})",
                    &id.id,
                    &id.span.start(),
                    &id.span.end()
                );
            }
        }
    }
}
