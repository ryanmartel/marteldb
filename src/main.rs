use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;

fn main() {
    let source = "SELECT tab.col, tab.col2, tab.col3 FROM tab1, tab2 WHERE this AND  NOT that;
    -- this is a comment";
    let lexer = Lexer::new(source);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);

}

