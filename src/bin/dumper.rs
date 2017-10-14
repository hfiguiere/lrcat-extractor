extern crate lrcat;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use lrcat::{Catalog,Keyword,LrObject};

use docopt::Docopt;

const USAGE: &'static str = "
Usage:
  dumper <command> ([--all] | [--collections] [--images] [--folders] [--keywords]) <path>

Options:
    --all          Select all objects
    --collections  Select only collections
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

        let keywords = catalog.load_keywords();
        println!("\tKeywords count: {}", keywords.len());

        if args.flag_keywords {
            dump_keywords(&keywords);
        }
    }
}

fn dump_keywords(keywords: &Vec<Keyword>) {
    println!("Keywords");
    println!("+--------+--------------------------------------+--------+----------------------------+");
    for keyword in keywords {
        println!("+ {:>6} + {} + {:>6} + {:<26} +", keyword.id(), keyword.uuid(), keyword.parent, keyword.name);
    }
    println!("+--------+--------------------------------------+--------+----------------------------+");
}

fn process_audit(_: &Args) {

}
