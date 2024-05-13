use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::StatementParser;

fn main() {
    let source = "INSERT dave;";
    let lexer = Lexer::new(source);
    let parser = StatementParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);

}

