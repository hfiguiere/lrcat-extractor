
use rusqlite::Connection;
use rusqlite;

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
}
