
use rusqlite::Connection;
use rusqlite;

use folders::{Folders,Folder,RootFolder};
use fromdb::FromDb;
use images::Image;
use keyword::Keyword;
use libraryfile::LibraryFile;


const LR3_VERSION: i32 = 3;
const LR4_VERSION: i32 = 4;

/// Catalog version.
#[derive(Debug,PartialEq)]
pub enum CatalogVersion {
    Unknown,
    Lr3,
    Lr4
}

pub struct Catalog {
    /// catalog path
    path: String,
    pub version: String,
    pub catalog_version: CatalogVersion,
    pub root_keyword_id: f64,

    keywords: Vec<Keyword>,
    folders: Folders,
    images: Vec<Image>,
    libfiles: Vec<LibraryFile>,

    dbconn: Option<Connection>,
}

impl Catalog {

    pub fn new(path: &str) -> Catalog {
        Catalog {
            path: String::from(path),
            version: String::from(""),
            catalog_version: CatalogVersion::Unknown,
            root_keyword_id: 0.0,
            keywords: vec!(),
            folders: Folders::new(),
            images: vec!(),
            libfiles: vec!(),
            dbconn: None
        }
    }

    pub fn open(&mut self) -> bool {
        let conn_attempt = Connection::open(&self.path);
        if let Ok(conn) = conn_attempt {
            self.dbconn = Some(conn);

            return true;
        }

        false
    }

    pub fn get_variable<T>(&self, name: &str) -> Option<T>
        where T: rusqlite::types::FromSql {

        if let Some(ref conn) = self.dbconn {
            if let Ok(mut stmt) = conn.prepare("SELECT value FROM Adobe_variablesTable WHERE name=?1") {
                let mut rows = stmt.query(&[&name]).unwrap();
                if let Some(Ok(row)) = rows.next() {
                    return Some(row.get(0));
                }
            }
        }
        None
    }

    fn parse_version(mut v: String) -> i32 {
        v.truncate(2);
        if let Ok(version) = v.parse::<i32>() {
            version
        } else {
            0
        }
    }

    pub fn load_version(&mut self) {
        if let Some(version) = self.get_variable::<String>("Adobe_DBVersion") {
            self.version = version;
            let v = Catalog::parse_version(self.version.clone());
            self.catalog_version = match v {
                LR4_VERSION =>
                    CatalogVersion::Lr4,
                LR3_VERSION =>
                    CatalogVersion::Lr3,
                _ =>
                    CatalogVersion::Unknown
            };
        }

        if let Some(root_keyword_id) =
            self.get_variable::<f64>("AgLibraryKeyword_rootTagID") {
                self.root_keyword_id = root_keyword_id;
            }
    }

    pub fn load_objects<T: FromDb>(conn: &Connection) -> Vec<T> {
        let mut result: Vec<T> = vec!();
        let query = format!("SELECT {} FROM {}",
                            T::read_db_columns(),
                            T::read_db_tables());
        if let Ok(mut stmt) = conn.prepare(&query) {
            let mut rows = stmt.query(&[]).unwrap();
            while let Some(Ok(row)) = rows.next() {
                if let Some(object) = T::read_from(&row) {
                    result.push(object);
                }
            }
        }
        result
    }

    pub fn load_keywords(&mut self) -> &Vec<Keyword> {
        if self.keywords.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let mut result = Catalog::load_objects::<Keyword>(&conn);
                self.keywords.append(&mut result);
            }
        }
        return &self.keywords;
    }

    pub fn load_folders(&mut self) -> &Folders {
        if self.folders.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let folders = Catalog::load_objects::<RootFolder>(&conn);
                self.folders.append_root_folders(folders);
                let mut folders = Catalog::load_objects::<Folder>(&conn);
                for ref mut folder in &mut folders {
                    folder.content = Some(folder.read_content(conn));
                }
                self.folders.append_folders(folders);
            }
        }
        return &self.folders;
    }

    pub fn load_library_files(&mut self) -> &Vec<LibraryFile> {
        if self.libfiles.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let mut result = Catalog::load_objects::<LibraryFile>(&conn);
                self.libfiles.append(&mut result);
            }
        }
        return &self.libfiles;
    }

    pub fn load_images(&mut self) -> &Vec<Image> {
        if self.images.is_empty() {
            if let Some(ref conn) = self.dbconn {
                let mut result = Catalog::load_objects::<Image>(&conn);
                self.images.append(&mut result);
            }
        }
        return &self.images;
    }
}
