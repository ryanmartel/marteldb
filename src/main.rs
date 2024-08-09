use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;
use marteldb::parser::prettyprinter::PrettyPrinter;
use marteldb::parser::errors;
use marteldb::repl;
use marteldb::parser::visitor::*;

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

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
                        let ast_res = parser.parse(lexer);
                        match ast_res {
                            Ok(ast) => {
                                for i in &ast {
                                    let mut pp = PrettyPrinter::new();
                                    pp.visit_stmt(i);
                                }
                            }
                            Err(err) => {
                                match err {
                                    lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                                        errors.push(errors::Error::ParseError(errors::Item::new(token.0..token.2, token.1.to_string())));
                                    }
                                    _ => {}
                                }
                            }
                        }
                        let config = codespan_reporting::term::Config::default();
                        let writer = StandardStream::stderr(ColorChoice::Always);
                        let file = SimpleFile::new("input.sql", &line);
                        for diagnostic in errors.iter().map(errors::Error::report) {
                            term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
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
