use std::fmt::Display;

use ast::name::Name;
use source_index::{location::Location, span::Span};

#[derive(PartialEq, Clone, Debug, Eq, Copy)]
pub struct Token {
    /// The kind of token.
    kind: TokenKind,
    /// span of the token.
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
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

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Name,
    TableAll,
    TableCol,

    // Keywords -------------
    Abort,
    Action,
    Add,
    All,
    Alter,
    And,
    As,
    Asc,
    Begin,
    Between,
    By,
    Cascade,
    Check,
    Collate,
    Column,
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
    Fail,
    From,
    Full,
    Group,
    Having,
    If,
    Ignore,
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
    No,
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
    Release,
    Rename,
    Replace,
    Restrict,
    Right,
    Rollback,
    Savepoint,
    Select,
    Set,
    Table,
    To,
    Transaction,
    Union,
    Unique,
    Update,
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
    // matches '/'
    Slash,
    // matches '*'
    Star,
    // matches '%'
    Percent,
    // matches '='
    Equals,
    // matches >
    Greater,
    // matches >=
    GreaterEqual,
    // matches <
    Less,
    // matches <=
    LessEqual,
    // matches !
    Exclamation,
    // matches != or <>
    NotEqual,
    // matches ','
    Comma,

    // SQL Types
    Char,
    Integer,
    Numeric,
    Serial,
    Varchar,

    // Interior Types
    Float,
    Int,
    String,

    Comment,
    Unknown,
    EndOfFile,
}

impl TokenKind {

    pub fn is_type(self) -> bool {
        matches!(
            self,
            TokenKind::Char
            | TokenKind::Integer
            | TokenKind::Numeric
            | TokenKind::Serial
            | TokenKind::Varchar
        )
    }
    
    pub fn is_column_constraint(self) -> bool {
        matches!(
            self,
            TokenKind::Primary
            | TokenKind::Not
            | TokenKind::Unique
            | TokenKind::Check
            | TokenKind::Default
            | TokenKind::Collate
            | TokenKind::References
        )
    }
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
    TableAll(Name),
    TableCol(Name, Name),

    Float(f64),
    Int(i64),
    String(String),
}
