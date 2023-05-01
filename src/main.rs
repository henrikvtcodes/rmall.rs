use clap::Parser;
use deleteutil::{delete_dir, delete_file, has_nested_folder};
use pathutil::parse_path;
use std::path::PathBuf;

mod deleteutil;
mod pathutil;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Delete nested directories
    #[arg(short, long, default_value_t = false)]
    recursive: bool,

    /// Ignore invalid files & directories
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// Output what is being removed
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// The paths to files and directories that should be deleted
    #[command()]
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for input in args.paths {
        let path = parse_path(&input);
        match path {
            Ok(pb) => handle_delete(pb, args.recursive, args.verbose, args.force),
            Err(_e) => {
                if !args.force {
                    panic!("Invalid Path: {}", input)
                }
            }
        }
    }
}

fn handle_delete(path: PathBuf, recurse: bool, verbose: bool, force: bool) {
    if path.is_file() {
        let succeeded = delete_file(path.as_path());
        if verbose {
            if succeeded {
                println!("Deleted file at {:?}", path.to_string_lossy())
            } else {
                println!("Failed to delete file at {:?}", path.to_string_lossy())
            }
        }
    } else if path.is_dir() {
        let succeeded = delete_dir(path.as_path(), recurse);
        if verbose {
            if succeeded {
                println!("Deleted directory at {:?}", path.to_string_lossy())
            } else {
                let failed_for_recurse = recurse && has_nested_folder(path.as_path());
                if failed_for_recurse {
                    println!(
                        "Failed to delete directory at {:?}. Remove contents.",
                        path.to_string_lossy()
                    )
                } else {
                    println!("Failed to delete directory at {:?}", path.to_string_lossy())
                }
            }
        }
    } else {
        if !force {
            panic!("Unknown path type at {:?}", path.to_string_lossy())
        }
    }
}
