
use rusqlite::Connection;
use rusqlite;

use folders::{Folders,Folder,RootFolder};
use fromdb::FromDb;
use keyword::Keyword;


const LR4_VERSION: &str = "0400020";

/// Catalog version.
#[derive(Debug)]
pub enum CatalogVersion {
    Unknown,
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

    pub fn load_version(&mut self) {
        if let Some(version) = self.get_variable::<String>("Adobe_DBVersion") {
            self.version = version;
            if self.version == LR4_VERSION {
                self.catalog_version = CatalogVersion::Lr4;
            }
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
                let folders = Catalog::load_objects::<Folder>(&conn);
                self.folders.append_folders(folders);
            }
        }
        return &self.folders;
    }
}
