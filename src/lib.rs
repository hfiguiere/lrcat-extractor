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
#[derive(Debug)]
pub enum Error {
    /// Skip the item (when reading from Db)
    Skip,
    /// Unsupported catalog version
    UnsupportedVersion,
    /// Sql Error
    Sql(rusqlite::Error),
    /// Lron parsing error
    Lron(peg::error::ParseError<peg::str::LineCol>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Skip => write!(f, "LrCat: Skip."),
            Self::UnsupportedVersion => write!(f, "LrCat: Unsupported catalog version."),
            Self::Sql(ref e) => write!(f, "LrCat: SQL error: {}", e),
            Self::Lron(ref e) => write!(f, "LrCat: Lron parsing error: {}", e),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Sql(ref err) => Some(err),
            Self::Lron(ref err) => Some(err),
            Self::Skip | Self::UnsupportedVersion => None,
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Sql(err)
    }
}

impl From<peg::error::ParseError<peg::str::LineCol>> for Error {
    fn from(err: peg::error::ParseError<peg::str::LineCol>) -> Self {
        Self::Lron(err)
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
