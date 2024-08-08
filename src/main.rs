use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;
use marteldb::parser::prettyprinter::PrettyPrinter;
use marteldb::repl;
use marteldb::parser::visitor::*;

use std::io::Write;



fn main() -> Result<(), String> {
    loop {
        let line = repl::readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match repl::respond(line) {
            Ok(resp) => {
                match resp {
                    repl::Response::Quit => {
                        break;
                    }
                    repl::Response::MetaCommand => {}
                    repl::Response::Stmt => {
                        let lexer = Lexer::new(line);
                        let parser = ScriptParser::new();
                        let ast = parser.parse(lexer).unwrap();

                        for i in &ast {
                            let mut pp = PrettyPrinter::new();
                            pp.visit_stmt(i);
                        }
                    }
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}
