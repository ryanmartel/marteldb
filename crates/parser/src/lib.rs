// use lalrpop_util::lalrpop_mod;
// lalrpop_mod!(pub grammar);

use parser::{Parsed, Parser};
use parsing_ast::Script;

pub mod parser;
pub mod visitor;
pub mod statement;

pub fn parse_script(source: &str) -> Parsed<Script> {
    Parser::new(source)
        .parse()
}
