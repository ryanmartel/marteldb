use parsing_ast::ident;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token {
    // Kind of Token
    kind: TokenKind,
    // Range of token.
    range: (u32, u32)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum TokenKind {

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
