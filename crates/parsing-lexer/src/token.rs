use core::fmt;

use parsing_ast::ident::{Ident, TableCol, TableAll};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token {
    // Kind of Token
    kind: TokenKind,
    // Range of token.
    range: (u32, u32)
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.kind, self.range)?;
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum TokenKind {
    // Literals
    // A name
    Identifier,
    // Table.column
    TableCol,
    // Table.*
    TableAll,
    // i32
    Int,
    // String literal. single quoted
    String,
    False,
    Null,
    True,

    // Types
    Boolean,
    Numeric,
    Serial,
    Varchar,

    // KeyWords
    Abort,
    And,
    As,
    Asc,
    Begin,
    Cascade,
    Commit,
    Count,
    Create,
    Delete,
    Desc,
    Distinct,
    Drop,
    End,
    Exists,
    From,
    Having,
    Index,
    Insert,
    Into,
    Limit,
    Not,
    OnDelete,
    Or,
    OrderBy,
    PrimaryKey,
    References,
    Rollback,
    Select,
    Set,
    Table,
    Update,
    Vacuum,
    Values,
    Where,



    Comment,
    EndOfFile,
    /// Token kind for a question mark `?`.
    Question,
    /// Token kind for an exclamation mark `!`.
    Exclamation,
    /// Token kind for a left parenthesis `(`.
    Lpar,
    /// Token kind for a right parenthesis `)`.
    Rpar,
    /// Token kind for a left square bracket `[`.
    Lsqb,
    /// Token kind for a right square bracket `]`.
    Rsqb,
    /// Token kind for a comma `,`.
    Comma,
    /// Token kind for a semicolon `;`.
    Semi,
    /// Token kind for plus `+`.
    Plus,
    /// Token kind for minus `-`.
    Minus,
    /// Token kind for star `*`.
    Star,
    /// Token kind for slash `/`.
    Slash,
    /// Token kind for vertical bar `|`.
    Vbar,
    /// Token kind for ampersand `&`.
    Amper,
    /// Token kind for less than `<`.
    Less,
    /// Token kind for greater than `>`.
    Greater,
    /// Token kind for equal `=`.
    Equal,
    /// Token kind for dot `.`.
    Dot,
    /// Token kind for percent `%`.
    Percent,
    /// Token kind for double equal `==`.
    EqEqual,
    /// Token kind for not equal `!=` or `<>`.
    NotEqual,
    /// Token kind for less than or equal `<=`.
    LessEqual,
    /// Token kind for greater than or equal `>=`.
    GreaterEqual,
    /// Token kind for left shift `<<`.
    LeftShift,
    /// Token kind for right shift `>>`.
    RightShift,

    Newline,
    Unknown
}

#[derive(Clone, Debug, Default)]
pub enum TokenValue {
    #[default]
    None,

    Int(i32),

    // Identifier
    Identifier(Ident),

    // Table.column
    TableCol(TableCol),

    // Table.*
    TableAll(TableAll),

    // String Literal
    String(Box<str>),
}
