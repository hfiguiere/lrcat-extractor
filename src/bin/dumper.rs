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
    /// Dump the catalog.
    Dump(CommandArgs),
    /// Audit (unimplemented).
    Audit(CommandArgs),
}

#[derive(Debug, Parser)]
struct CommandArgs {
    /// Path to the catalog.
    path: PathBuf,
    /// Dump everything.
    #[arg(long)]
    all: bool,
    /// Dump collections.
    #[arg(long)]
    collections: bool,
    /// Dump library files.
    #[arg(long)]
    libfiles: bool,
    /// Dump images.
    #[arg(long)]
    images: bool,
    /// Dump folders.
    #[arg(long)]
    folders: bool,
    /// Dump root folders.
    #[arg(long)]
    root: bool,
    /// Dump keywords.
    #[arg(long)]
    keywords: bool,
}

#[derive(Debug, Parser)]
struct ListArgs {
    /// The catalog
    path: PathBuf,
    /// Sort
    #[arg(short)]
    sort: bool,
    /// Only list directories
    #[arg(short)]
    dirs: bool,
}

fn main() -> lrcat::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::List(ref args) => process_list(args),
        Command::Dump(_) => process_dump(&args),
        Command::Audit(_) => process_audit(&args),
    }
}

fn list_dirs(folders: &BTreeMap<LrId, String>, sort: bool) {
    let mut folders = folders.values().collect::<Vec<&String>>();
    if sort {
        folders.sort_unstable();
    }
    folders.iter().for_each(|folder| println!("{}", folder));
}

fn list_files(catalog: &mut Catalog, folders: &BTreeMap<LrId, String>, sort: bool) {
    let libfiles = catalog.load_library_files();
    let mut files = libfiles
        .iter()
        .filter_map(|file| {
            folders.get(&file.folder).map(|folder| {
                let mut out = vec![format!("{}{}.{}", folder, file.basename, file.extension)];
                out.extend(file.sidecar_extensions.split(',').filter_map(|ext| {
                    if !ext.is_empty() {
                        Some(format!("{}{}.{}", folder, file.basename, ext))
                    } else {
                        None
                    }
                }));

                out
            })
        })
        .flatten()
        .collect::<Vec<String>>();

    if sort {
        files.sort_unstable();
    }
    files.iter().for_each(|file| println!("{}", file));
}

fn process_list(args: &ListArgs) -> lrcat::Result<()> {
    let mut catalog = Catalog::new(&args.path);
    catalog.open()?;
    let folders = catalog.load_folders();

    let roots = BTreeMap::from_iter(
        folders
            .roots
            .iter()
            .map(|folder| (folder.id(), folder.clone())),
    );

    let resolved_folders = BTreeMap::from_iter(folders.folders.iter().map(|folder| {
        let root_path = if let Some(root) = roots.get(&folder.root_folder) {
            &root.absolute_path
        } else {
            ""
        };

        (
            folder.id(),
            format!("{}{}", root_path, &folder.path_from_root),
        )
    }));

    if args.dirs {
        list_dirs(&resolved_folders, args.sort);
    } else {
        list_files(&mut catalog, &resolved_folders, args.sort);
    }

    Ok(())
}

fn process_dump(args: &Args) -> lrcat::Result<()> {
    if let Command::Dump(args) = &args.command {
        let mut catalog = Catalog::new(&args.path);
        catalog.open()?;

        catalog.load_version();
        println!("Catalog:");
        println!(
            "\tVersion: {} ({:?})",
            catalog.version, catalog.catalog_version
        );
        println!("\tRoot keyword id: {}", catalog.root_keyword_id);

        if !catalog.catalog_version.is_supported() {
            println!("Unsupported catalog version");
            return Err(lrcat::Error::UnsupportedVersion);
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
    Ok(())
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

fn process_audit(_: &Args) -> lrcat::Result<()> {
    Err(lrcat::Error::Unimplemented)
}
