use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;

fn main() {
    let source = "INSERT dave;
    -- this is a comment
    Insert dan;";
    let lexer = Lexer::new(source);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);

}

