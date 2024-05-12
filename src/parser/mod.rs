mod token;
mod lexer;
mod ast;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator1, "/parser/calculator1.rs");

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
}
