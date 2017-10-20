/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::Row;

use fromdb::FromDb;
use lrobject::LrObject;

pub struct Image {
    id: i64,
    uuid: String,
    pub master_image: Option<i64>,
    pub copy_name: Option<String>,
    pub rating: Option<i64>,
    pub root_file: i64,
    pub file_format: String,
    pub pick: i64,
    pub orientation: Option<String>,
    pub capture_time: String,
}

impl LrObject for Image {
    fn id(&self) -> i64 {
        self.id
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl FromDb for Image {
    fn read_from(row: &Row) -> Option<Self> {
        Some(Image {
            id: row.get(0),
            uuid: row.get(1),
            master_image: row.get_checked(2).ok(),
            rating: row.get_checked(3).ok(),
            root_file: row.get(4),
            file_format: row.get(5),
            pick: row.get(6),
            orientation: row.get_checked(7).ok(),
            capture_time: row.get(8),
            copy_name: row.get_checked(9).ok(),
        })
    }
    fn read_db_tables() -> &'static str {
        "Adobe_images"
    }
    fn read_db_columns() -> &'static str {
        "id_local,id_global,masterImage,rating,rootFile,fileFormat,cast(pick as integer),orientation,captureTime,copyName"
    }
}
