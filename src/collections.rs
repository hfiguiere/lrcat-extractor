/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use rusqlite::{Connection, Row};

use crate::content::Content;
use crate::fromdb::FromDb;
use crate::lrobject::LrId;

/// A collection as defined in Lightroom
pub struct Collection {
    /// Local id of the collection
    id: LrId,
    /// Name of the collection (displayed in the UI)
    pub name: String,
    /// Parent of the `Collection`
    pub parent: LrId,
    /// is system only (seems to be the Quick Pick collection)
    pub system_only: bool,
    /// content definition of the collection
    pub content: Option<Content>,
}

impl FromDb for Collection {
    fn read_from(row: &Row) -> rusqlite::Result<Self> {
        Ok(Collection {
            id: row.get(0)?,
            name: row.get(2)?,
            parent: row.get(3).unwrap_or(0),
            system_only: match row.get::<usize, f64>(4)? as i64 {
                0 => false,
                _ => true,
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
    pub fn id(&self) -> LrId {
        self.id
    }

    /// Read the `content` for this collection from the database.
    pub fn read_content(&self, conn: &Connection) -> Content {
        Content::from_db(conn, "AgLibraryCollectionContent", "collection", self.id)
    }
}
