/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::{Connection, Row};

use content::Content;
use fromdb::FromDb;
use lrobject::{LrId, LrObject};

/// A folder define the container for `LibraryFiles`
/// They are all attached to a `RootFolder`
pub struct Folder {
    id: LrId,
    uuid: String,
    /// Path from the `RootFolder`
    pub path_from_root: String,
    /// Id of the `RootFolder`
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
        Folder {
            id,
            uuid: String::from(uuid),
            path_from_root: String::from(""),
            root_folder: 0,
            content: None,
        }
    }
    pub fn read_content(&self, conn: &Connection) -> Content {
        Content::from_db(conn, "AgFolderContent", "containingFolder", self.id)
    }
}

/// Represent the ancestor of `Folder` and map to
/// an absolute path
pub struct RootFolder {
    id: LrId,
    uuid: String,
    /// Absolute path of the `RootFolder`
    pub absolute_path: String,
    /// (User readable) name of the `RootFolder`
    pub name: String,
    /// Eventually if it is possible the path is relative
    /// to the catalog file.
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
    /// Create a new `RootFolder` with an id and uuid
    pub fn new(id: LrId, uuid: &str) -> RootFolder {
        RootFolder {
            id,
            uuid: String::from(uuid),
            absolute_path: String::from(""),
            name: String::from(""),
            relative_path_from_catalog: None,
        }
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

/// Represent the all the folders
pub struct Folders {
    /// The `RootFolder` list
    pub roots: Vec<RootFolder>,
    /// The `Folder` list
    pub folders: Vec<Folder>,
}

impl Folders {
    /// Create an empty `Folders`
    pub fn new() -> Folders {
        Folders {
            roots: vec![],
            folders: vec![],
        }
    }

    /// Return `true` is it is empty
    pub fn is_empty(&self) -> bool {
        self.roots.is_empty() && self.folders.is_empty()
    }

    /// Add a `Folder`
    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
    }

    /// Add a `RootFolder`
    pub fn add_root_folder(&mut self, root_folder: RootFolder) {
        self.roots.push(root_folder);
    }

    /// Append a vector of `Folder`
    pub fn append_folders(&mut self, mut folders: Vec<Folder>) {
        self.folders.append(&mut folders);
    }

    /// Append a vector of `RootFolder`
    pub fn append_root_folders(&mut self, mut root_folders: Vec<RootFolder>) {
        self.roots.append(&mut root_folders);
    }

    /// Return the eventual `RootFolder` with the id.
    fn find_root_folder(&self, id: LrId) -> Option<&RootFolder> {
        for root in &self.roots {
            if root.id() == id {
                return Some(root);
            }
        }
        None
    }

    /// Resolve the folder path by providing an absolute path
    /// This does not check if the path exist but merely combine
    /// the `RootFolder` absolute_path and the `Folder` relative path
    pub fn resolve_folder_path(&self, folder: &Folder) -> Option<String> {
        let root_folder = try_opt!(self.find_root_folder(folder.root_folder));
        let mut root_path = root_folder.absolute_path.clone();
        root_path += &folder.path_from_root;
        return Some(root_path);
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
