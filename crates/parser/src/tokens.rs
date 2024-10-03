use std::fmt::Display;

use ast::name::Name;
use source_index::{location::Location, span::Span};

pub struct Token {
    /// The kind of token.
    kind: TokenKind,
    /// span of the token.
    span: Span,
}

impl Token {

    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token {
            kind,
            span,
        }
    }

    pub fn start(&self) -> Location {
        self.span.start()
    }

    pub fn end(&self) -> Location {
        self.span.end()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {

    Name,


    // Keywords -------------
    Abort,
    All,
    Alter,
    And,
    As,
    Asc,
    Begin,
    Between,
    By,
    Cascade,
    Collate,
    Commit,
    Conflict,
    Constraint,
    Create,
    Cross,
    Default,
    Delete,
    Desc,
    Distinct,
    Drop,
    Except,
    Exists,
    From,
    Full,
    Group,
    Having,
    In,
    Index,
    Indexed,
    Inner,
    Insert,
    Is,
    Join,
    Key,
    Left,
    Like,
    Limit,
    Natural,
    Not,
    Null,
    Offset,
    On,
    Or,
    Order,
    Outer,
    Primary,
    References,
    Reindex,
    Rename,
    Right,
    Rollback,
    Savepoint,
    Select,
    Set,
    Table,
    Transaction,
    Union,
    Unique,
    Using,
    Vacuum,
    Values,
    Where,

    // matches '('
    LParen,
    // matches ')'
    RParen,
    // matches ';'
    Semicolon,
    // matches '-'
    Minus,
    // matches '+'
    Plus,
    // matches '='
    Equals,

    // Types
    Float,
    Int,
    String,


    Comment,
    Unknown,
    EndOfFile,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum TokenValue {
    #[default]
    None,

    Name(Name),
    Float(f64),
    Int(i32),
    String(String),

}
