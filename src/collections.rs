/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::{Connection,Row};

use content::Content;
use fromdb::FromDb;

/// A collection as defined in Lightroom
pub struct Collection {
    id: i64,
    pub name: String,
    pub parent: i64,
    /// is system only (seems to be the quick pick)
    pub system_only: bool,
    /// content definition of the collection
    pub content: Option<Content>,
}

impl FromDb for Collection {
    fn read_from(row: &Row) -> Option<Self> {
        Some(Collection {
            id: row.get(0),
            name: row.get(2),
            parent: row.get_checked(3).unwrap_or(0),
            system_only: match row.get::<i32,f64>(4) as i64 {
                0 => false,
                _ => true
            },
            content: None,
        })
    }
    fn read_db_tables() -> &'static str {
        "AgLibraryCollection"
    }
    fn read_db_columns() -> &'static str {
        "id_local,genealogy,name,parent,systemOnly"
    }
}

impl Collection {
    /// Return the local_id of the collection.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Read the `content` for this collection from the database.
    pub fn read_content(&self, conn: &Connection) -> Content {
        Content::from_db(conn, "AgLibraryCollectionContent",
                         "collection", self.id)
    }
}
