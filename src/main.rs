use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;
use marteldb::parser::prettyprinter::PrettyPrinter;
use marteldb::parser::errors;
use marteldb::repl;
use marteldb::parser::visitor::*;

use std::io::Write;
use std::path::Path;



fn main() -> Result<(), String> {
    println!("Welcome to MartelDB. For a list of commands, enter 'help'");
    if !Path::new("marteldb_data").exists() {
        println!("No data directory found!");
        println!("Initialize one with 'init [path]'")
    }
    loop {
        let line = repl::readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // TODO: ("This can probably be a closure");
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
                        let mut errors = Vec::new();
                        let ast_res = parser.parse(&mut errors, lexer);
                        match ast_res {
                            Ok(ast) => {
                                for i in &ast {
                                    let mut pp = PrettyPrinter::new();
                                    pp.visit_stmt(i);
                                }
                                for error in errors {
                                    match error.error {
                                        lalrpop_util::ParseError::UnrecognizedToken{token, expected} => {
                                            println!("token-start: {}, end {}", token.0, token.2);
                                            println!("{:?}", expected);
                                        }
                                        _ => {}
                                    }
                                    // println!("{:?}", error.error);
                                }
                            }
                            Err(err) => {
                                println!("{}", err);
                            }
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
