use std::fs;
use std::env;
use std::path::{PathBuf, Path};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Delete nested directories
    #[arg(short, long, default_value_t=false)]
    recursive: bool,

    /// Ignore invalid files & directories
    #[arg(short, long, default_value_t=false)]
    force: bool,

    /// Output what is being removed
    #[arg(short, long, default_value_t=false)]
    verbose: bool

}

fn main() {
    let args = Args::parse();

    println!("Force: {:?}", args.force);
    println!("Verbose: {:?}", args.verbose);
    println!("Recurse: {:?}", args.recursive);

}
    


fn has_nested_folder(path: &Path) -> bool {
    let mut has_nested = false;
    for entry in fs::read_dir(path).expect("Error with read_dir") {
        let entry = entry.expect("Error with entry");
        let path = entry.path();
        if path.is_dir() {
            has_nested = true;
            break;
        }
    }
    has_nested
}