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
    #[token("CREATE", ignore(case))]
    Create,
    #[token("DROP", ignore(case))]
    Drop,
    #[token("TABLE", ignore(case))]
    Table,
    #[token("INSERT", ignore(case))]
    Insert,
    #[token("INTO", ignore(case))]
    Into,
    #[token("VALUES", ignore(case))]
    Values,
    #[token("SELECT", ignore(case))]
    Select,
    #[token("FROM", ignore(case))]
    From,
    #[token("DELETE", ignore(case))]
    Delete,
    #[token("UPDATE", ignore(case))]
    Update,
    #[token("SET", ignore(case))]
    Set,
    #[token("WHERE", ignore(case))]
    Where,
    #[token("ORDER BY", ignore(case))]
    OrderBy,
    #[token("NOT", ignore(case))]
    Not,
    #[token("AND", ignore(case))]
    And,
    #[token("OR", ignore(case))]
    Or,
    #[token("PRIMARY KEY", ignore(case))]
    PrimaryKey,
    #[token("REFERENCES", ignore(case))]
    References,
    #[token("ON DELETE", ignore(case))]
    OnDelete,
    #[token("CASCADE", ignore(case))]
    Cascade,


    // Types
    #[token("INT", ignore(case))]
    TypeInt,
    #[token("VARCHAR", ignore(case))]
    Varchar,
    #[token("NUMERIC", ignore(case))]
    Numeric,
    #[token("NULL", ignore(case))]
    Null,


    #[regex("[a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    Integer(i32),

    // ETC Terminals
    #[token("*")]
    All,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    #[token("'")]
    SingleQuote,
    #[token("\"")]
    DoubleQuote,
    #[token("=")]
    Equals,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
