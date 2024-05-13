mod token;
mod lexer;
mod ast;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator1, "/parser/calculator1.rs");
lalrpop_mod!(pub grammar, "/parser/grammar.rs");

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
}

#[test]
fn grammar() {
    let source = "INSERT dave;";
    let lexer = Lexer::new(source);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);
}
