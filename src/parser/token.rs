use logos::Logos;
use std::fmt;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"--.*\n?", error = LexicalError)]
pub enum Token {

    // KEYWORDS
    #[token("ABORT", ignore(case))] Abort,
    #[token("AND", ignore(case))] And,
    #[token("AS", ignore(case))] As,
    #[token("ASC", ignore(case))] Asc,
    #[token("BEGIN", ignore(case))] Begin,
    #[token("CASCADE", ignore(case))] Cascade,
    #[token("COMMIT", ignore(case))] Commit,
    #[token("COUNT", ignore(case))] Count,
    #[token("CREATE", ignore(case))] Create,
    #[token("DELETE", ignore(case))] Delete,
    #[token("DESC", ignore(case))] Desc,
    #[token("DISTINCT", ignore(case))] Distinct,
    #[token("DROP", ignore(case))] Drop_,
    #[token("END", ignore(case))] End,
    #[token("FROM", ignore(case))] From_,
    #[token("HAVING", ignore(case))] Having,
    #[token("INDEX", ignore(case))] Index,
    #[token("INSERT", ignore(case))] Insert,
    #[token("INTO", ignore(case))] Into_,
    #[token("LIMIT", ignore(case))] Limit,
    #[token("NOT", ignore(case))] Not,
    #[token("ON DELETE", ignore(case))] OnDelete,
    #[token("OR", ignore(case))] Or,
    #[token("ORDER BY", ignore(case))] OrderBy,
    #[token("PRIMARY KEY", ignore(case))] PrimaryKey,
    #[token("REFERENCES", ignore(case))] References,
    #[token("ROLLBACK", ignore(case))] Rollback,
    #[token("SELECT", ignore(case))] Select,
    #[token("SET", ignore(case))] Set,
    #[token("TABLE", ignore(case))] Table,
    #[token("UPDATE", ignore(case))] Update,
    #[token("VACUUM", ignore(case))] Vacuum,
    #[token("VALUES", ignore(case))] Values,
    #[token("WHERE", ignore(case))] Where,

    // Types
    #[token("BOOLEAN", ignore(case))] Boolean,
    #[token("INT", ignore(case))] TypeInt,
    #[token("NULL", ignore(case))] Null,
    #[token("NUMERIC", ignore(case))] Numeric,
    #[token("SERIAL", ignore(case))] Serial,
    #[token("VARCHAR", ignore(case))] Varchar,


    #[regex("[a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    Integer(i32),

    // ETC Terminals
    #[token("*")] All,
    #[token(";")] Semicolon,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token(",")] Comma,
    #[token(".")] Period,
    #[token("'")] SingleQuote,
    #[token("\"")] DoubleQuote,
    #[token("=")] Equals,
    #[token("-")] Negative,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
