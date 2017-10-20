
use rusqlite::Row;

/// Trait to define loading from a database.
pub trait FromDb : Sized {

    /// Read one element from a database Row obtained through a query
    /// build with the tables and columns provided.
    fn read_from(row: &Row) -> Option<Self>;
    /// DB tables used in select query.
    fn read_db_tables() -> &'static str;
    /// DB columns used in select query.
    fn read_db_columns() -> &'static str;
}
