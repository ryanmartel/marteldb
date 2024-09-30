pub struct Identifier {

}

#[derive(Default, Debug)]
pub enum IdentifierKind {
    #[default]
    Simple,

    // Table-quantified Columns in form `table.col`
    TableCol,
    // Table-all colums in form `table.*`
    TableAll,
}
