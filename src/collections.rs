/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use rusqlite::{Connection, Row};

use crate::catalog::CatalogVersion;
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
    fn read_from(version: CatalogVersion, row: &Row) -> crate::Result<Self> {
        match version {
            CatalogVersion::Lr4 |
            CatalogVersion::Lr6 =>
                Ok(Collection {
                    id: row.get(0)?,
                    name: row.get(2)?,
                    parent: row.get(3).unwrap_or(0),
                    system_only: !matches!(row.get::<usize, f64>(4)? as i64, 0),
                    content: None,
                }),
            CatalogVersion::Lr2 => {
                let tag_type: Box<str> = row.get(3)?;
                let name: String = row.get(1)
                    .unwrap_or_else(|_| {
                        if tag_type.as_ref() == "AgQuickCollectionTagKind" {
                            "Quick Collection"
                        } else {
                            ""
                        }.to_owned()
                    });
                match tag_type.as_ref() {
                    "AgQuickCollectionTagKind" |
                    "AgCollectionTagKind" =>
                        Ok(Collection {
                            id: row.get(0)?,
                            name,
                            parent: row.get(2).unwrap_or(0),
                            system_only: matches!(tag_type.as_ref(), "AgQuickCollectionTagKind"),
                            content: None,
                        }),
                    _ => Err(crate::Error::Skip)

                }
            }
            _ => Err(crate::Error::UnsupportedVersion)
        }
    }

    fn read_db_tables(version: CatalogVersion) -> &'static str {
        match version {
            CatalogVersion::Lr4 |
            CatalogVersion::Lr6 =>
                "AgLibraryCollection",
            CatalogVersion::Lr2 =>
                "AgLibraryTag",
            _ => ""
        }
    }

    fn read_db_columns(version: CatalogVersion) -> &'static str {
        match version {
            CatalogVersion::Lr4 |
            CatalogVersion::Lr6 =>
                "id_local,genealogy,name,parent,systemOnly",
            CatalogVersion::Lr2 =>
                "id_local,name,parent,kindName",
            _ => ""
        }
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
