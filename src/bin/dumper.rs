extern crate lrcat;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use lrcat::{Catalog,CatalogVersion,Folders,Image,Keyword,LibraryFile,LrObject};

use docopt::Docopt;

const USAGE: &'static str = "
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
    arg_path: String,
    flag_all: bool,
    flag_collections: bool,
    flag_libfiles: bool,
    flag_images: bool,
    flag_folders: bool,
    flag_keywords: bool
}

#[derive(Debug, Deserialize)]
enum Command {
    Dump,
    Audit,
    Unknown(String)
}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());
    {
        match args.arg_command {
            Command::Dump =>
                process_dump(&args),
            Command::Audit =>
                process_audit(&args),
            _ =>
                ()
        };
    }
}

fn process_dump(args: &Args) {
    let mut catalog = Catalog::new(&args.arg_path);
    if catalog.open() {
        catalog.load_version();
        println!("Catalog:");
        println!("\tVersion: {} ({:?})", catalog.version,
                 catalog.catalog_version);
        println!("\tRoot keyword id: {}", catalog.root_keyword_id);

        if catalog.catalog_version != CatalogVersion::Lr4 {
            println!("Unsupported catalog version");
            return;
        }
        {
            let keywords = catalog.load_keywords();
            println!("\tKeywords count: {}", keywords.len());

            if args.flag_all || args.flag_keywords {
                dump_keywords(&keywords);
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
    }
}

fn dump_keywords(keywords: &Vec<Keyword>) {
    println!("Keywords");
    println!("+--------+--------------------------------------+--------+----------------------------");
    println!("+ id     + uuid                                 + parent + name");
    println!("+--------+--------------------------------------+--------+----------------------------");
    for keyword in keywords {
        println!("+ {:>6} + {} + {:>6} + {:<26}", keyword.id(), keyword.uuid(), keyword.parent, keyword.name);
    }
    println!("+--------+--------------------------------------+--------+----------------------------");
}

fn dump_folders(folders: &Folders) {
    println!("Root Folders");
    println!("+--------+--------------------------------------+------------------+----------------------------");
    println!(" id      + uuid                                 + name             + absolute path");
    println!("+--------+--------------------------------------+------------------+----------------------------");
    for root in &folders.roots {
        println!("+ {:>6} + {} + {:<16} + {:<26}", root.id(), root.uuid(), root.name, root.absolute_path);
    }
    println!("+--------+--------------------------------------+------------------+----------------------------");
    println!("Folders");
    println!("+--------+--------------------------------------+--------+----------------------------");
    println!("+ id     + uuid                                 + root   + path");
    println!("+--------+--------------------------------------+--------+----------------------------");
    for folder in &folders.folders {
        println!("+ {:>6} + {} + {:>6} + {:<26} + {:?}",
                 folder.id(), folder.uuid(), folder.root_folder,
                 folder.path_from_root, folder.content);
    }
    println!("+--------+--------------------------------------+--------+----------------------------");
}

fn dump_libfiles(libfiles: &Vec<LibraryFile>) {
    println!("Images");
    println!("+--------+--------------------------------------+--------+--------+------------------+----------+");
    println!(" id      + uuid                                 + folder + extens + basename         + sidecars +");
    println!("+--------+--------------------------------------+--------+--------+------------------+----------+");
    for libfile in libfiles {
        println!("+ {:>6} + {} + {:>6} + {:<6} + {:<16} + {:<8} +",
                 libfile.id(), libfile.uuid(), libfile.folder,
                 libfile.extension, libfile.basename,
                 libfile.sidecar_extensions);
    }
    println!("+--------+--------------------------------------+--------+--------+------------------+----------+");
}

fn dump_images(images: &Vec<Image>) {
    println!("Images");
    println!("+--------+--------------------------------------+--------+--------+----+----+");
    println!(" id      + uuid                                 + root   + format + or + P +");
    println!("+--------+--------------------------------------+--------+--------+----+----+");
    for image in images {
        println!("+ {:>6} + {} + {:>6} + {:<6} + {:<2} + {} +",
                 image.id(), image.uuid(), image.root_file,
                 image.file_format,
                 image.orientation.as_ref().unwrap_or(&String::from("")),
                 image.pick);
    }
    println!("+--------+--------------------------------------+--------+--------+----+----+");

}

fn process_audit(_: &Args) {

}
