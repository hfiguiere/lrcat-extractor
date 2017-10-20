/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::{Row,Connection};

use content::Content;
use fromdb::FromDb;
use lrobject::LrObject;

pub struct Folder {
    id: i64,
    uuid: String,
    pub path_from_root: String,
    pub root_folder: i64,
    pub content: Option<Content>,
}

impl LrObject for Folder {
    fn id(&self) -> i64 {
        self.id
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl FromDb for Folder {
    fn read_from(row: &Row) -> Option<Self> {
        Some(Folder {
            id: row.get(0),
            uuid: row.get(1),
            path_from_root: row.get(2),
            root_folder: row.get(3),
            content: None,
        })
    }
    fn read_db_tables() -> &'static str {
        "AgLibraryFolder"
    }
    fn read_db_columns() -> &'static str {
        "id_local,id_global,pathFromRoot,rootFolder"
    }
}

impl Folder {
    pub fn read_content(&self, conn: &Connection) -> Content {
        Content::from_db(conn, "AgFolderContent", "containingFolder", self.id)
    }
}

pub struct RootFolder {
    id: i64,
    uuid: String,
    pub absolute_path: String,
    pub name: String,
    pub relative_path_from_catalog: Option<String>,
}

impl LrObject for RootFolder {
    fn id(&self) -> i64 {
        self.id
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl FromDb for RootFolder {
    fn read_from(row: &Row) -> Option<Self> {
        Some(RootFolder {
            id: row.get(0),
            uuid: row.get(1),
            absolute_path: row.get(2),
            name: row.get(3),
            relative_path_from_catalog: row.get_checked(4).ok(),
        })
    }

    fn read_db_tables() -> &'static str {
        "AgLibraryRootFolder"
    }

    fn read_db_columns() -> &'static str {
        "id_local,id_global,absolutePath,name,relativePathFromCatalog"
    }

}

pub struct Folders {
    pub roots: Vec<RootFolder>,
    pub folders: Vec<Folder>,
}

impl Folders {

    pub fn new() -> Folders {
        Folders { roots: vec!(),
                  folders: vec!() }
    }

    pub fn is_empty(&self) -> bool {
        self.roots.is_empty() && self.folders.is_empty()
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
    }

    pub fn add_root_folder(&mut self, root_folder: RootFolder) {
        self.roots.push(root_folder);
    }

    pub fn append_folders(&mut self, mut folders: Vec<Folder>) {
        self.folders.append(&mut folders);
    }

    pub fn append_root_folders(&mut self, mut root_folders: Vec<RootFolder>) {
        self.roots.append(&mut root_folders);
    }

}
