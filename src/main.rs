// use marteldb::parser::lexer::Lexer;
// use marteldb::parser::grammar::ScriptParser;
// use marteldb::parser::prettyprinter::PrettyPrinter;
// use marteldb::parser::visitor::*;
//
use std::io::Write;
use marteldb::repl;


fn main() -> Result<(), String> {
    loop {
        let line = repl::readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match repl::respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())

    // let source = "SELECT tab.col, tab.col2, tab.col3 FROM tab1, tab2 WHERE this AND  NOT that;
    // -- this is a comment
    // INSERT INTO tab1(col1, col2, col3) VALUES ('a', 10, FALSE);
    // CREATE TABLE tab3();
    // INSERT INTO tab2 VALUES('b', 20, NULL);";
    // let lexer = Lexer::new(source);
    // let parser = ScriptParser::new();
    // let ast = parser.parse(lexer).unwrap();
    //
    // for i in &ast {
    //     let mut pp = PrettyPrinter::new();
    //     pp.visit_stmt(i);
    //     // println!("{}", 1);
    // }
    // println!("{:#?}", ast);

}

