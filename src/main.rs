use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;
use marteldb::parser::prettyprinter::PrettyPrinter;
use marteldb::parser::visitor::*;

fn main() {
    let source = "SELECT tab.col, tab.col2, tab.col3 FROM tab1, tab2 WHERE this AND  NOT that;
    -- this is a comment
    INSERT INTO tab1(col1, col2, col3) VALUES ('a', 10, FALSE);
    INSERT INTO tab2 VALUES('b', 20, NULL);";
    let lexer = Lexer::new(source);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    for i in &ast {
        let mut pp = PrettyPrinter::new();
        pp.visit_stmt(i);
        // println!("{}", 1);
    }
    // println!("{:#?}", ast);

}

