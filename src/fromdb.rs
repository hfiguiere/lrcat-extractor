/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use rusqlite::Row;

use crate::catalog::CatalogVersion;

/// Trait to define loading from a database.
pub trait FromDb: Sized {
    /// Read one element from a database Row obtained through a query
    /// build with the tables and columns provided.
    /// The version of the catalog allow selecting the proper variant.
    fn read_from(version: CatalogVersion, row: &Row) -> crate::Result<Self>;
    /// DB tables used in select query.
    fn read_db_tables(version: CatalogVersion) -> &'static str;
    /// DB columns used in select query.
    fn read_db_columns(version: CatalogVersion) -> &'static str;
}
