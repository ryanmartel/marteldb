use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;
use marteldb::parser::prettyprinter::PrettyPrinter;
use marteldb::parser::visitor::*;
use clap::{Parser, Subcommand};
//
use std::io::Write;

pub fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "MartelDB > ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}


fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match respond(line) {
            Ok(resp) => {
                match resp {
                    Response::Quit => {
                        break;
                    }
                    Response::MetaCommand => {}
                    Response::Stmt => {
                        let lexer = Lexer::new(line);
                        let parser = ScriptParser::new();
                        let ast = parser.parse(lexer).unwrap();

                        for i in &ast {
                            let mut pp = PrettyPrinter::new();
                            pp.visit_stmt(i);
                            // println!("{}", 1);
                        }
                    }
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
        // let v: Vec<&str> = line.split(' ').collect();
        // if line.eq("exit") {
        //     break;
        // }

        // let lexer = Lexer::new(line);
        // let parser = ScriptParser::new();
        // let ast = parser.parse(lexer).unwrap();
        //
        // for i in &ast {
        //     let mut pp = PrettyPrinter::new();
        //     pp.visit_stmt(i);
        //     // println!("{}", 1);
        // }
    }

    Ok(())
}

fn respond(line: &str) -> Result<Response, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli_res = Cli::try_parse_from(args).map_err(|e| e.to_string());
    match cli_res {
        Ok(cli) => {
            match cli.command {
                Commands::Ping => {
                    write!(std::io::stdout(), "Pong").map_err(|e| e.to_string())?;
                    std::io::stdout().flush().map_err(|e| e.to_string())?;
                }
                Commands::Exit => {
                    write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
                    std::io::stdout().flush().map_err(|e| e.to_string())?;
                    return Ok(Response::Quit);
                }
            }
        },
        // No metacommand
        Err(_) => {
            return Ok(Response::Stmt);
        }
    }
    Ok(Response::MetaCommand)
}

enum Response {
    Quit,
    MetaCommand,
    Stmt
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ping,
    Exit,
}
    // let source = "SELECT tab.col, tab.col2, tab.col3 FROM tab1, tab2 WHERE this AND  NOT that;
    // -- this is a comment
    // INSERT INTO tab1(col1, col2, col3) VALUES ('a', 10, FALSE);
    // CREATE TABLE tab3();
    // INSERT INTO tab2 VALUES('b', 20, NULL);";
    // let lexer = Lexer::new(source);
    // let parser = ScriptParser::new();
    // let ast = parser.parse(lexer).unwrap();
    //
    // for i in &ast {
    //     let mut pp = PrettyPrinter::new();
    //     pp.visit_stmt(i);
    //     // println!("{}", 1);
    // }
    // println!("{:#?}", ast);


