/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::Row;

use fromdb::FromDb;
use lrobject::{LrId,LrObject};

pub struct LibraryFile {
    id: LrId,
    uuid: String,
    pub basename: String,
    pub extension: String,
    pub folder: LrId,
    pub sidecar_extensions: String,
}

impl LrObject for LibraryFile {
    fn id(&self) -> LrId {
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
