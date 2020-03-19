/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt;

use rusqlite::Connection;

use lron;

#[derive(Debug)]
pub enum SortDirection {
    Ascending,
    Descending,
    Unknown,
}

#[derive(Default)]
/// Represent the content view. Applies to `Collection` and `Folder`
pub struct Content {
    /// Filter
    pub filter: Option<String>,
    /// What to sort on
    pub sort_type: Option<String>,
    /// Which direction to sort
    pub sort_direction: Option<SortDirection>,
    /// Define the smart collection (if any)
    pub smart_collection: Option<lron::Object>,
}

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut empty: bool = true;
        if let Some(ref filter) = self.filter {
            write!(f, "filter: {:?}", filter)?;
            empty = false;
        }
        if let Some(ref sort_type) = self.sort_type {
            if !empty {
                write!(f, ", ")?;
            }
            write!(f, "sort: {:?}", sort_type)?;
            empty = false;
        }
        if let Some(ref direction) = self.sort_direction {
            if !empty {
                write!(f, ", ")?;
            }
            write!(f, "direction: {:?}", direction)?;
            empty = false;
        }
        if let Some(ref smart_coll) = self.smart_collection {
            if !empty {
                write!(f, ", ")?;
            }
            write!(f, "smart_collection: {:?}", smart_coll)?;
        }
        Ok(())
    }
}

impl Content {
    pub fn from_db(
        conn: &Connection,
        table: &str,
        container_col: &str,
        container_id: i64,
    ) -> Content {
        let mut content = Content::default();

        let query = format!(
            "SELECT content,owningModule from {} where {}=?1",
            table, container_col
        );
        if let Ok(mut stmt) = conn.prepare(&query) {
            let mut rows = stmt.query(&[&container_id]).unwrap();
            while let Some(Ok(row)) = rows.next() {
                let value = row.get_checked(0);
                let owning_module: String = row.get(1);
                match owning_module.as_str() {
                    "com.adobe.ag.library.filter" => content.filter = value.ok(),
                    "com.adobe.ag.library.sortType" => content.sort_type = value.ok(),
                    "com.adobe.ag.library.sortDirection" => {
                        content.sort_direction = if let Ok(sd) = value {
                            match sd.as_str() {
                                "ascending" => Some(SortDirection::Ascending),
                                "descending" => Some(SortDirection::Descending),
                                _ => Some(SortDirection::Unknown),
                            }
                        } else {
                            None
                        }
                    }
                    "ag.library.smart_collection" => {
                        if let Ok(ref sc) = value {
                            content.smart_collection = lron::Object::from_string(sc).ok();
                        }
                    }
                    _ => (),
                };
            }
        }
        content
    }
}
