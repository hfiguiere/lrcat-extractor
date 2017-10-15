
use rusqlite::Row;

use fromdb::FromDb;
use lrobject::LrObject;

pub struct LibraryFile {
    id: i64,
    uuid: String,
    pub basename: String,
    pub extension: String,
    pub folder: i64,
    pub sidecar_extensions: String,
}

impl LrObject for LibraryFile {
    fn id(&self) -> i64 {
        self.id
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl FromDb for LibraryFile {
    fn read_from(row: &Row) -> Option<Self> {
        Some(LibraryFile {
            id: row.get(0),
            uuid: row.get(1),
            basename: row.get(2),
            extension: row.get(3),
            folder: row.get(4),
            sidecar_extensions: row.get(5),
        })
    }
    fn read_db_tables() -> &'static str {
        "AgLibraryFile"
    }
    fn read_db_columns() -> &'static str {
        "id_local,id_global,baseName,extension,folder,sidecarExtensions"
    }
}

impl LibraryFile {

}
