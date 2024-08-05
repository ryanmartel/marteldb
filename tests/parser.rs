use marteldb::parser::lexer::Lexer;
use marteldb::parser::grammar::ScriptParser;
use marteldb::parser::ast::*;

pub struct TestParser<'input> {
    pub lexer: Lexer<'input>,
    pub parser: ScriptParser
}

fn create_ast(source: &str) -> Vec<Stmt> {

    let lexer = Lexer::new(source);
    let parser = ScriptParser::new();
    parser.parse(lexer).expect("Failed to parse this stmt")
}

#[test]
fn test_parses() {
    let source = " --
        -- Comments
        -- This is a comment
        -- ; = 2@1! // /**/ -- --- -- @@##### This is a weird comment

        -- Select Statements
        SELECT col1 FROM tab1;
        SeLeCt col1, col2 FrOm tab1, tab2;
        SELECT * FROM tab1;
        SELECT tab1.* FROM tab1;
        SELECT tab1.col1 FROM tab1 WHERE 1 = 1;
        SELECT col1 FROM tab1 WHERE col1 > 2;

        -- Insert Statements
        INSERT INTO tab1 VALUES(1, 'fred', TRUE, NULL);
        INSERT INTO tab1 SELECT * FROM tab1;
    ";
    let _ast = create_ast(source);
}



