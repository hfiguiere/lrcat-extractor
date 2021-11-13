/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate docopt;
extern crate lrcat;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::path::PathBuf;

use docopt::Docopt;

use lrcat::{
    Catalog, Collection, Folders, Image, Keyword, KeywordTree, LibraryFile, LrId, LrObject,
};

const USAGE: &str = "
Usage:
  dumper <command> ([--all] | [--collections] [--libfiles] [--images] [--folders] [--keywords]) <path>

Options:
    --all          Select all objects
    --collections  Select only collections
    --libfiles     Select only library files
    --images       Select only images
    --folders      Select only folders
    --keywords     Select only keywords

Commands are:
    dump           Dump the objects
    audit          Audit mode: output what we ignored
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_command: Command,
    arg_path: PathBuf,
    flag_all: bool,
    flag_collections: bool,
    flag_libfiles: bool,
    flag_images: bool,
    flag_folders: bool,
    flag_keywords: bool,
}

#[derive(Debug, Deserialize)]
enum Command {
    Dump,
    Audit,
    Unknown(String),
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());
    {
        match args.arg_command {
            Command::Dump => process_dump(&args),
            Command::Audit => process_audit(&args),
            _ => (),
        };
    }
}

fn process_dump(args: &Args) {
    let mut catalog = Catalog::new(&args.arg_path);
    if catalog.open() {
        catalog.load_version();
        println!("Catalog:");
        println!(
            "\tVersion: {} ({:?})",
            catalog.version, catalog.catalog_version
        );
        println!("\tRoot keyword id: {}", catalog.root_keyword_id);

        if !catalog.catalog_version.is_supported() {
            println!("Unsupported catalog version");
            return;
        }

        {
            let root_keyword_id = catalog.root_keyword_id;
            let keywordtree = catalog.load_keywords_tree();
            let keywords = catalog.load_keywords();
            println!("\tKeywords count: {}", keywords.len());

            if args.flag_all || args.flag_keywords {
                dump_keywords(root_keyword_id, &keywords, &keywordtree);
            }
        }

        {
            let folders = catalog.load_folders();
            if args.flag_all || args.flag_folders {
                dump_folders(&folders);
            }
        }

        {
            let libfiles = catalog.load_library_files();
            if args.flag_all || args.flag_libfiles {
                dump_libfiles(&libfiles);
            }
        }
        {
            let images = catalog.load_images();
            if args.flag_all || args.flag_images {
                dump_images(&images);
            }
        }
        {
            let collections = catalog.load_collections();
            if args.flag_all || args.flag_collections {
                dump_collections(&collections);
            }
        }
    }
}

fn print_keyword(level: i32, id: LrId, keywords: &BTreeMap<i64, Keyword>, tree: &KeywordTree) {
    if let Some(keyword) = keywords.get(&id) {
        let mut indent = String::from("");
        if level > 0 {
            for _ in 0..level - 1 {
                indent.push(' ');
            }
            indent.push_str("+ ")
        }
        println!(
            "| {:>7} | {} | {:>7} | {}{}",
            keyword.id(),
            keyword.uuid(),
            keyword.parent,
            indent,
            keyword.name
        );
        let children = tree.children_for(id);
        for child in children {
            print_keyword(level + 1, child, keywords, tree);
        }
    }
}

fn dump_keywords(root: LrId, keywords: &BTreeMap<i64, Keyword>, tree: &KeywordTree) {
    println!("Keywords");
    println!(
        "+---------+--------------------------------------+---------+----------------------------"
    );
    println!("| id      | uuid                                 | parent  | name");
    println!(
        "+---------+--------------------------------------+---------+----------------------------"
    );
    print_keyword(0, root, keywords, tree);
    println!(
        "+---------+--------------------------------------+---------+----------------------------"
    );
}

fn dump_folders(folders: &Folders) {
    println!("Root Folders");
    println!("+---------+--------------------------------------+------------------+----------------------------");
    println!("| id      | uuid                                 | name             | absolute path");
    println!("+---------+--------------------------------------+------------------+----------------------------");
    for root in &folders.roots {
        println!(
            "| {:>7} | {} | {:<16} | {:<26}",
            root.id(),
            root.uuid(),
            root.name,
            root.absolute_path
        );
    }
    println!("+---------+--------------------------------------+------------------+----------------------------");
    println!("Folders");
    println!("+---------+--------------------------------------+--------+-----------------------------+----------");
    println!(
        "| id      | uuid                                 | root   | path                        |"
    );
    println!("+---------+--------------------------------------+--------+-----------------------------+----------");
    for folder in &folders.folders {
        println!(
            "| {:>7} | {} | {:>7} | {:<26} | {:?}",
            folder.id(),
            folder.uuid(),
            folder.root_folder,
            folder.path_from_root,
            folder.content
        );
    }
    println!("+---------+--------------------------------------+--------+-----------------------------+----------");
}

fn dump_libfiles(libfiles: &[LibraryFile]) {
    println!("Libfiles");
    println!("+---------+--------------------------------------+---------+--------+---------------------+----------+");
    println!("| id      | uuid                                 | folder  | extens | basename            | sidecars |");
    println!("+---------+--------------------------------------+---------+--------+---------------------+----------+");
    for libfile in libfiles {
        println!(
            "| {:>7} | {} | {:>7} | {:<6} | {:<19} | {:<8} |",
            libfile.id(),
            libfile.uuid(),
            libfile.folder,
            libfile.extension,
            libfile.basename,
            libfile.sidecar_extensions
        );
    }
    println!("+---------+--------------------------------------+---------+--------+---------------------+----------+");
}

fn dump_images(images: &[Image]) {
    println!("Images");
    println!("+---------+--------------------------------------+---------+--------+-------+----+-----------");
    println!(
        "| id      | uuid                                 | root    | format | or    | P  | xmp "
    );
    println!("+---------+--------------------------------------+---------+--------+-------+----+-----------");
    for image in images {
        println!(
            "| {:>7} | {} | {:>7} | {:<6} | {:<2}({}) | {:>2} | {} bytes ",
            image.id(),
            image.uuid(),
            image.root_file,
            image.file_format,
            image.orientation.as_ref().unwrap_or(&String::new()),
            image.exif_orientation(),
            image.pick,
            image.xmp.len(),
        );
    }
    println!("+---------+--------------------------------------+---------+--------+-------+----+-----------");
}

fn dump_collections(collections: &[Collection]) {
    println!("Collections");
    println!("+---------+--------------------------------------+---------+-------+----------------------");
    println!("| id      | name                                 | parent  | syst  | content");
    println!("+---------+--------------------------------------+---------+-------+----------------------");
    for collection in collections {
        println!(
            "| {:>7} | {:<36} | {:>7} | {:<5} | {:?}",
            collection.id(),
            collection.name,
            collection.parent,
            collection.system_only,
            collection.content
        )
    }
    println!("+---------+--------------------------------------+---------+-------+----------------------");
}

fn process_audit(_: &Args) {}
