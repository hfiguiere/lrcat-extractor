
use rusqlite::Row;

pub trait FromDb : Sized {

    fn read_from(row: &Row) -> Option<Self>;
    fn read_db_tables() -> &'static str;
    fn read_db_columns() -> &'static str;

}
