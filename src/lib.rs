/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate chrono;
extern crate peg;
extern crate rusqlite;

mod catalog;
mod collections;
mod content;
mod folders;
mod fromdb;
mod images;
mod keywords;
mod keywordtree;
mod libraryfiles;
mod lrobject;
pub mod lron;

/// Error from the crate, agreggate with sqlite errors.
#[derive(Debug)]
pub enum Error {
    /// Skip the item (when reading from Db)
    Skip,
    /// Unsupported catalog version
    UnsupportedVersion,
    /// Sql Error
    Sql(rusqlite::Error)
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Sql(err)
    }
}

/// Result type for the crate.
pub type Result<T> = std::result::Result<T, Error>;

pub use catalog::{Catalog, CatalogVersion};
pub use collections::Collection;
pub use content::Content;
pub use folders::{Folder, Folders, RootFolder};
pub use images::Image;
pub use keywords::Keyword;
pub use keywordtree::KeywordTree;
pub use libraryfiles::LibraryFile;
pub use lrobject::{LrId, LrObject};
