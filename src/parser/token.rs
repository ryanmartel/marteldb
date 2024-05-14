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
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = LexicalError)]
pub enum Token {

    // KEYWORDS
    #[token("CREATE", ignore(case))]
    Create,
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


    // Types
    #[token("INT", ignore(case))]
    TypeInt,


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
    #[token("'")]
    SingleQuote,
    #[token("=")]
    Equals,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
