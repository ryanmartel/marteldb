use std::io::Write;
use clap::error::ErrorKind;
use clap::{Parser, Subcommand};

pub fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "MartelDB > ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

pub fn respond(line: &str) -> Result<Response, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli_res = Cli::try_parse_from(args);
    match cli_res {
        Ok(cli) => {
            match cli.command {
                Commands::Ping => {
                    writeln!(std::io::stdout(), "Pong").map_err(|e| e.to_string())?;
                    std::io::stdout().flush().map_err(|e| e.to_string())?;
                }
                Commands::Init { name } => {
                    writeln!(std::io::stdout(), "Initializing database {name}").map_err(|e| e.to_string())?;
                    std::io::stdout().flush().map_err(|e| e.to_string())?;
                }
                Commands::Exit => {
                    write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
                    std::io::stdout().flush().map_err(|e| e.to_string())?;
                    return Ok(Response::Quit);
                }
            }
        },
        // Not valid metacommand
        Err(err) => {
            match err.kind() {
                ErrorKind::InvalidSubcommand => {
                    return Ok(Response::Stmt);
                }
                ErrorKind::UnknownArgument => {
                    return Ok(Response::Stmt);
                }
                // Other type of CLI error, such as missing arguments to metacommand
                _ => {
                    return Err(err.to_string());
                }
            }
        }
    }
    Ok(Response::MetaCommand)
}

#[derive(Debug)]
pub enum Response {
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
    Init {
        name: String,
    },
    Exit,
}
