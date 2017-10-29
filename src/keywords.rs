/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::Row;

use fromdb::FromDb;
use lrobject::LrObject;

pub struct Keyword {
    id: i64,
    uuid: String,
//  date_created: DateTime<Utc>,
    pub name: String,
    pub parent: i64
}

impl LrObject for Keyword {
    fn id(&self) -> i64 {
        self.id
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl FromDb for Keyword {

    fn read_from(row: &Row) -> Option<Self> {
        let name = row.get_checked(3);
        let parent = row.get_checked(4);
        Some(Keyword {
            id: row.get(0),
            uuid: row.get(1),
            name: name.unwrap_or(String::from("")),
            parent: parent.unwrap_or(0),
        })
    }

    fn read_db_tables() -> &'static str {
        return "AgLibraryKeyword";
    }

    fn read_db_columns() -> &'static str {
        return "id_local,id_global,cast(dateCreated as text),name,parent";
    }
}

impl Keyword {
    /// Initialize a new keyword.
    pub fn new(id: i64, parent: i64, uuid: &str, name: &str) -> Keyword {
        Keyword { id, parent, uuid: String::from(uuid), name: String::from(name) }
    }
}
