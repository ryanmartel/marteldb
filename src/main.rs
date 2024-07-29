use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;

fn main() {
    let source = "SELECT tab.col, tab.col2, tab.col3;
    -- this is a comment";
    let lexer = Lexer::new(source);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);

}

