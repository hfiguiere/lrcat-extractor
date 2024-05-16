/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate lrcat;

use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use lrcat::{
    Catalog, Collection, Folders, Image, Keyword, KeywordTree, LibraryFile, LrId, LrObject,
};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List content of the catalog.
    List(ListArgs),
    Dump(CommandArgs),
    Audit(CommandArgs),
}

#[derive(Debug, Parser)]
struct CommandArgs {
    path: PathBuf,
    #[arg(long)]
    all: bool,
    #[arg(long)]
    collections: bool,
    #[arg(long)]
    libfiles: bool,
    #[arg(long)]
    images: bool,
    #[arg(long)]
    folders: bool,
    #[arg(long)]
    root: bool,
    #[arg(long)]
    keywords: bool,
}

#[derive(Debug, Parser)]
struct ListArgs {
    /// The catalog
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::List(ref args) => process_list(args),
        Command::Dump(_) => process_dump(&args),
        Command::Audit(_) => process_audit(&args),
    };
}

fn process_list(args: &ListArgs) {
    let mut catalog = Catalog::new(&args.path);
    if catalog.open() {
        let folders = catalog.load_folders();

        let roots = BTreeMap::from_iter(
            folders.roots.iter().map(|folder| (folder.id(), folder.clone()))
        );
        for folder in &folders.folders {
            let root_path = if let Some(root) = roots.get(&folder.root_folder) {
                &root.absolute_path
            } else {
                ""
            };

            println!(
                "{}{}",
                root_path,
                &folder.path_from_root,
            );
        }
    }
}

fn process_dump(args: &Args) {
    if let Command::Dump(args) = &args.command {
        let mut catalog = Catalog::new(&args.path);
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

                if args.all || args.keywords {
                    dump_keywords(root_keyword_id, keywords, &keywordtree);
                }
            }

            {
                let folders = catalog.load_folders();
                if args.all || args.root {
                    dump_root_folders(folders);
                }
                if args.all || args.folders {
                    dump_folders(folders);
                }
            }

            {
                let libfiles = catalog.load_library_files();
                if args.all || args.libfiles {
                    dump_libfiles(libfiles);
                }
            }
            {
                let images = catalog.load_images();
                if args.all || args.images {
                    dump_images(images);
                }
            }
            {
                let collections = catalog.load_collections();
                if args.all || args.collections {
                    dump_collections(collections);
                }
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

fn dump_root_folders(folders: &Folders) {
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
}

fn dump_folders(folders: &Folders) {
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
