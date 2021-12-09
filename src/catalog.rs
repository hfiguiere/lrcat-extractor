/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rusqlite::{params, Connection};

use crate::collections::Collection;
use crate::folders::{Folder, Folders, RootFolder};
use crate::fromdb::FromDb;
use crate::images::Image;
use crate::keywords::Keyword;
use crate::keywordtree::KeywordTree;
use crate::libraryfiles::LibraryFile;
use crate::lrobject::{LrId, LrObject};

const LR2_VERSION: i32 = 2;
const LR3_VERSION: i32 = 3;
const LR4_VERSION: i32 = 4;
const LR6_VERSION: i32 = 6;

/// Catalog version.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CatalogVersion {
    /// Unknown version
    Unknown,
    /// Lightroom 2.x catalog.
    Lr2,
    /// Lightroom 3.x catalog. Unhandled.
    Lr3,
    /// Lightroom 4.x catalog.
    Lr4,
    /// Lightroom 6.x catalog.
    Lr6,
}

impl CatalogVersion {
    /// Return if we support this catalog version
    pub fn is_supported(&self) -> bool {
        (*self == Self::Lr2) || (*self == Self::Lr3) || (*self == Self::Lr4) || (*self == Self::Lr6)
    }
}

/// Catalog is the main container for Lightroom. It represents
/// the .lrcat database.
pub struct Catalog {
    /// Catalog path
    path: PathBuf,
    /// The version string
    pub version: String,
    /// The catalog version
    pub catalog_version: CatalogVersion,
    /// Id for the root (top level) keyword
    pub root_keyword_id: LrId,

    /// The keywords, mapped in the local `LrId`
    keywords: BTreeMap<LrId, Keyword>,
    /// The `Folders` container.
    folders: Folders,
    /// The `Image` container
    images: Vec<Image>,
    /// The `LibraryFile` container
    libfiles: Vec<LibraryFile>,
    /// The `Collection` container
    collections: Vec<Collection>,

    /// The sqlite connectio to the catalog
    dbconn: Option<Connection>,
}

impl Catalog {
    /// Create a new catalog.
    pub fn new<P>(path: P) -> Catalog
    where
        P: AsRef<Path>,
    {
        Catalog {
            path: path.as_ref().to_path_buf(),
            version: "".to_string(),
            catalog_version: CatalogVersion::Unknown,
            root_keyword_id: 0,
            keywords: BTreeMap::new(),
            folders: Folders::new(),
            images: vec![],
            libfiles: vec![],
            collections: vec![],
            dbconn: None,
        }
    }

    /// Open catalog. Return false in failure.
    /// This doesn't check if the content is valid beyond the backing sqlite3.
    pub fn open(&mut self) -> bool {
        let conn_attempt = Connection::open(&self.path);
        if let Ok(conn) = conn_attempt {
            self.dbconn = Some(conn);

            return true;
        }

        false
    }

    /// Get a variable from the table.
    fn get_variable<T>(&self, name: &str) -> Option<T>
    where
        T: rusqlite::types::FromSql,
    {
        let conn = self.dbconn.as_ref()?;
        if let Ok(mut stmt) = conn.prepare("SELECT value FROM Adobe_variablesTable WHERE name=?1") {
            let mut rows = stmt.query(&[&name]).unwrap();
            if let Ok(Some(row)) = rows.next() {
                return row.get(0).ok();
            }
        }
        None
    }

    /// Pare the version string from the database.
    fn parse_version(mut v: String) -> i32 {
        v.truncate(2);
        if let Ok(version) = v.parse::<i32>() {
            version
        } else {
            0
        }
    }

    /// Load version info for the catalog.
    pub fn load_version(&mut self) {
        if let Some(version) = self.get_variable::<String>("Adobe_DBVersion") {
            self.version = version;
            let v = Catalog::parse_version(self.version.clone());
            self.catalog_version = match v {
                LR6_VERSION => CatalogVersion::Lr6,
                LR4_VERSION => CatalogVersion::Lr4,
                LR3_VERSION => CatalogVersion::Lr3,
                LR2_VERSION => CatalogVersion::Lr2,
                _ => CatalogVersion::Unknown,
            };
        }

        if let Some(root_keyword_id) = self.get_variable::<f64>("AgLibraryKeyword_rootTagID") {
            self.root_keyword_id = root_keyword_id.round() as LrId;
        }
    }

    /// Generic object loader leveraging the FromDb protocol
    fn load_objects<T: FromDb>(conn: &Connection, catalog_version: CatalogVersion) -> Vec<T> {
        let mut query = format!(
            "SELECT {} FROM {}",
            T::read_db_columns(catalog_version),
            T::read_db_tables(catalog_version)
        );
        let where_join = T::read_join_where(catalog_version);
        if !where_join.is_empty() {
            query += &format!(" WHERE {}", where_join);
        }
        if let Ok(mut stmt) = conn.prepare(&query) {
            if let Ok(rows) =
                stmt.query_and_then(params![], |row| T::read_from(catalog_version, row))
            {
                return rows
                    .into_iter()
                    .filter(|ref obj| obj.is_ok())
                    .map(|obj| obj.unwrap())
                    .collect();
            }
        }
        vec![]
    }

    /// Load a keyword tree
    pub fn load_keywords_tree(&mut self) -> KeywordTree {
        let keywords = self.load_keywords();

        let mut tree = KeywordTree::new();
        tree.add_children(keywords);

        tree
    }

    /// Load keywords.
    pub fn load_keywords(&mut self) -> &BTreeMap<LrId, Keyword> {
        if self.keywords.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let result = Catalog::load_objects::<Keyword>(&conn, self.catalog_version);
                for keyword in result {
                    self.keywords.insert(keyword.id(), keyword);
                }
            }
        }
        &self.keywords
    }

    /// Get the keywords. This assume the keywords have been loaded first.
    /// This allow non-mutable borrowing that would be caused by `load_keywords()`.
    pub fn keywords(&self) -> &BTreeMap<LrId, Keyword> {
        &self.keywords
    }

    /// Load folders.
    pub fn load_folders(&mut self) -> &Folders {
        if self.folders.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let folders = Catalog::load_objects::<RootFolder>(&conn, self.catalog_version);
                self.folders.append_root_folders(folders);
                let mut folders = Catalog::load_objects::<Folder>(&conn, self.catalog_version);
                for folder in &mut folders {
                    folder.content = Some(folder.read_content(conn));
                }
                self.folders.append_folders(folders);
            }
        }
        &self.folders
    }

    /// Get the folders. This assume the folders have been loaded first.
    /// This allow non-mutable borrowing that would be caused by `load_folders()`.
    pub fn folders(&self) -> &Folders {
        &self.folders
    }

    /// Load library files (that back images)
    pub fn load_library_files(&mut self) -> &Vec<LibraryFile> {
        if self.libfiles.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let mut result = Catalog::load_objects::<LibraryFile>(&conn, self.catalog_version);
                self.libfiles.append(&mut result);
            }
        }
        &self.libfiles
    }

    /// Get the libfiles. This assume the libfiles have been loaded first.
    /// This allow non-mutable borrowing that would be caused by `load_libfiles()`.
    pub fn libfiles(&self) -> &Vec<LibraryFile> {
        &self.libfiles
    }

    /// Load images.
    pub fn load_images(&mut self) -> &Vec<Image> {
        if self.images.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let mut result = Catalog::load_objects::<Image>(&conn, self.catalog_version);
                self.images.append(&mut result);
            }
        }
        &self.images
    }

    /// Get the images. This assume the images have been loaded first.
    /// This allow non-mutable borrowing that would be caused by `load_images()`.
    pub fn images(&self) -> &Vec<Image> {
        &self.images
    }

    /// Load collectons.
    pub fn load_collections(&mut self) -> &Vec<Collection> {
        if self.collections.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let mut collections =
                    Catalog::load_objects::<Collection>(&conn, self.catalog_version);
                for collection in &mut collections {
                    collection.content = Some(collection.read_content(conn));
                }
                self.collections.append(&mut collections);
            }
        }
        &self.collections
    }

    /// Get the collections. This assume the collections have been loaded first.
    /// This allow non-mutable borrowing that would be caused by `load_collections()`.
    pub fn collections(&self) -> &Vec<Collection> {
        &self.collections
    }

    /// Lr2 use "Tags".
    const LR2_QUERY: &'static str =
        "SELECT image FROM AgLibraryTagImage WHERE tag = ?1 AND tagKind = \"AgCollectionTagKind\"";
    /// Lr3, Lr4 and Lr6 store the relation in `AgLibraryCollectionImage`
    const LR4_QUERY: &'static str =
        "SELECT image FROM AgLibraryCollectionImage WHERE collection = ?1";

    /// Collect images from collections using a specific query.
    fn images_for_collection_with_query(
        &self,
        query: &str,
        collection_id: LrId,
    ) -> super::Result<Vec<LrId>> {
        let conn = self.dbconn.as_ref().unwrap();
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map(&[&collection_id], |row| row.get::<usize, i64>(0))?;
        let mut ids = Vec::new();
        for id in rows {
            ids.push(id?);
        }
        Ok(ids)
    }

    /// Return the of images in the given collection.
    /// Not to be confused with Content.
    pub fn images_for_collection(&self, collection_id: LrId) -> super::Result<Vec<LrId>> {
        match self.catalog_version {
            CatalogVersion::Lr2 => {
                self.images_for_collection_with_query(Self::LR2_QUERY, collection_id)
            }
            CatalogVersion::Lr3 | CatalogVersion::Lr4 | CatalogVersion::Lr6 => {
                self.images_for_collection_with_query(Self::LR4_QUERY, collection_id)
            }
            _ => Err(super::Error::UnsupportedVersion),
        }
    }
}
