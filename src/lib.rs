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

/// Point
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Aspect ratio.
#[derive(Debug, PartialEq)]
pub struct AspectRatio {
    pub width: i32,
    pub height: i32,
}

/// Rectangle. Lr uses 0..1.0 for
/// crops rectangles.
#[derive(Debug, PartialEq)]
pub struct Rect {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

/// Error from the crate, agreggate with sqlite errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("LrCat: Skip.")]
    /// Skip the item (when reading from Db)
    Skip,
    #[error("LrCat: Unsupported catalog version.")]
    /// Unsupported catalog version
    UnsupportedVersion,
    #[error("LrCat: SQL error: {0}")]
    /// Sql Error
    Sql(#[from] rusqlite::Error),
    #[error("LrCat: Lron parsing error: {0}")]
    /// Lron parsing error
    Lron(#[from] peg::error::ParseError<peg::str::LineCol>),
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
