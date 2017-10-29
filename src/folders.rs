/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::{Row,Connection};

use content::Content;
use fromdb::FromDb;
use lrobject::{LrId,LrObject};

pub struct Folder {
    id: LrId,
    uuid: String,
    pub path_from_root: String,
    pub root_folder: LrId,
    pub content: Option<Content>,
}

impl LrObject for Folder {
    fn id(&self) -> LrId {
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
    pub fn new(id: LrId, uuid: &str) -> Folder {
        Folder { id, uuid: String::from(uuid), path_from_root: String::from(""),
                 root_folder: 0, content: None }
    }
    pub fn read_content(&self, conn: &Connection) -> Content {
        Content::from_db(conn, "AgFolderContent", "containingFolder", self.id)
    }
}

pub struct RootFolder {
    id: LrId,
    uuid: String,
    pub absolute_path: String,
    pub name: String,
    pub relative_path_from_catalog: Option<String>,
}

impl LrObject for RootFolder {
    fn id(&self) -> LrId {
        self.id
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl RootFolder {
    pub fn new(id: LrId, uuid: &str) -> RootFolder {
        RootFolder { id, uuid: String::from(uuid),
                     absolute_path: String::from(""),
                     name: String::from(""), relative_path_from_catalog: None }
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

    fn find_root_folder(&self, id: i64) -> Option<&RootFolder> {
        for root in &self.roots {
            if root.id() == id {
                return Some(root);
            }
        }
        None
    }

    pub fn resolve_folder_path(&self, folder: &Folder) -> Option<String> {
        if let Some(root_folder) = self.find_root_folder(folder.root_folder) {
            let mut root_path = root_folder.absolute_path.clone();
            root_path += &folder.path_from_root;
            return Some(root_path);
        }
        None
    }
}


#[cfg(test)]
#[test]
fn test_resolve_folder_path() {
    let mut folders = Folders::new();

    let mut rfolder = RootFolder::new(24, "toplevel");
    rfolder.absolute_path = String::from("/home/hub/Pictures");
    rfolder.name = String::from("Pictures");
    folders.add_root_folder(rfolder);

    let mut folder = Folder::new(42, "foobar");
    folder.root_folder = 24;
    folder.path_from_root = String::from("/2017/10");
    folders.add_folder(folder);

    let resolved = folders.resolve_folder_path(&folders.folders[0]);
    assert!(resolved.is_some());
    let resolved = resolved.unwrap();
    assert_eq!(resolved, "/home/hub/Pictures/2017/10");
}
