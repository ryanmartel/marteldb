use std::io::Write;
use clap::{Parser, Subcommand};

pub struct Repl {

}
impl Repl {

    fn new() -> Self {

    }

    fn handle_input (&self) -> Response {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match respond(line) {
            Ok(resp) => resp,
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

}
    // loop {
    //     let line = readline()?;
    //     let line = line.trim();
    //     if line.is_empty() {
    //         continue;
    //     }
    //     match respond(line) {
    //         Ok(resp) => {
    //             match resp {
    //                 Response::Quit => {
    //                     break;
    //                 }
    //                 Response::MetaCommand => {}
    //                 Response::Stmt => {
    //                     let lexer = Lexer::new(line);
    //                     let parser = ScriptParser::new();
    //                     let ast = parser.parse(lexer).unwrap();
    //
    //                     for i in &ast {
    //                         let mut pp = PrettyPrinter::new();
    //                         pp.visit_stmt(i);
    //                         // println!("{}", 1);
    //                     }
    //                 }
    //             }
    //         }
    //         Err(err) => {
    //             write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
    //             std::io::stdout().flush().map_err(|e| e.to_string())?;
    //         }
    //     }
    //     // let v: Vec<&str> = line.split(' ').collect();
    //     // if line.eq("exit") {
    //     //     break;
    //     // }
    //
    //     // let lexer = Lexer::new(line);
    //     // let parser = ScriptParser::new();
    //     // let ast = parser.parse(lexer).unwrap();
    //     //
    //     // for i in &ast {
    //     //     let mut pp = PrettyPrinter::new();
    //     //     pp.visit_stmt(i);
    //     //     // println!("{}", 1);
    //     // }
    // }
    //
    // Ok(())

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

pub enum Response {
    Quit,
    MetaCommand,
    Stmt(line)
}

#[derive(Parser, Debug)]
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

pub fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "MartelDB > ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
