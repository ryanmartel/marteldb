use clap::error::ErrorKind;
use clap::{Parser, Subcommand};

use super::parser::lexer::Lexer;
use super::parser::grammar::ScriptParser;
use super::parser::prettyprinter::PrettyPrinter;
use super::parser::parsing_errors::{Item, Error};
use super::parser::visitor::*;

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;

pub struct Repl;

impl Repl {

    pub fn new() -> Self {
        println!("Welcome to MartelDB. For a list of commands, enter 'help'");
        if !Path::new("marteldb_data").exists() {
            println!("No data directory found!");
            println!("Initialize one with 'init [path]'");
        }
        Repl
    }

    pub fn run(&self) -> Result<(), String> {
        loop {
            let line = readline()?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            // TODO: ("This can probably be a closure");
            match respond(line) {
                Ok(resp) => {
                    resp.respond()?;
                    if resp.is_quit() {
                        break;
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
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "MartelDB > ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn respond(line: &str) -> Result<Response, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli_res = Cli::try_parse_from(args);
    match cli_res {
        Ok(cli) => {
            match cli.command {
                Commands::Init => {
                    Ok(Response::MetaCommand(Commands::Init))
                }
                Commands::Exit => {
                    Ok(Response::Quit)
                }
                Commands::Source { file_path } => {
                    Ok(Response::MetaCommand(Commands::Source { file_path }))
                }
            }
        },
        // Not valid metacommand
        Err(err) => {
            match err.kind() {
                ErrorKind::InvalidSubcommand => {
                    return Ok(Response::Stmt(String::from(line)));
                }
                ErrorKind::UnknownArgument => {
                    return Ok(Response::Stmt(String::from(line)));
                }
                // Other type of CLI error, such as missing arguments to metacommand
                _ => {
                    return Err(err.to_string());
                }
            }
        }
    }
}

#[derive(Debug)]
enum Response {
    Quit,
    MetaCommand(Commands),
    Stmt(String)
}

impl Response {

    fn is_quit(&self) -> bool {
        match self {
            Self::Quit => true,
            _ => false,
        }
    }

    fn respond(&self) -> Result<(), String> {
        match self {
            Self::Quit => {
                write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
            Self::Stmt(line) => {
                parse_with_errors("STDIN",line, &mut PrettyPrinter::new());
            }
            Self::MetaCommand(command) => {
                match command {
                    Commands::Source {file_path} => {
                        let mut f = OpenOptions::new()
                            .read(true)
                            .open(Path::new(&file_path)).unwrap();
                        let mut contents  = String::new();
                        f.read_to_string(&mut contents).unwrap();
                        parse_with_errors(&file_path, &contents, &mut PrettyPrinter::new());
                    }
                    Commands::Init => {
                        writeln!(std::io::stdout(), "Initializing data directory").map_err(|e| e.to_string())?;
                        std::io::stdout().flush().map_err(|e| e.to_string())?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize the data directory
    Init,
    /// Exit the REPL
    Exit,
    /// Run the given sql file
    Source {
        file_path: String,
    },
}

fn parse_with_errors(source_name: &str, contents: &str, visitor: &mut impl Visitor) {
    let lexer = Lexer::new(contents);
    let parser = ScriptParser::new();
    let mut errors = Vec::new();
    let ast_res = parser.parse(lexer);
    match ast_res {
        Ok(ast) => {
            for i in &ast {
                visitor.visit_stmt(i);
            }
        }
        Err(err) => {
            match err {
                lalrpop_util::ParseError::UnrecognizedToken { token, .. } => {
                    errors.push(Error::ParseError(Item::new(token.0..token.2, token.1.to_string())));
                }
                _ => {}
            }
        }
    }
    let config = codespan_reporting::term::Config::default();
    let writer = StandardStream::stderr(ColorChoice::Always);
    let file = SimpleFile::new(source_name, contents);
    for diagnostic in errors.iter().map(Error::report) {
        term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
    }
}
