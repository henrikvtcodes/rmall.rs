use std::fs;
use std::env;
use std::io::Error;
use std::path::{PathBuf};


fn main() {

    for arg in env::args().skip(1) {
        let path_raw = PathBuf::from(arg);
        let real_path = path_raw.canonicalize().expect("Invalid Path");
        let file_metadata = fs::metadata(&real_path).expect("Error with Metadata");
    }
}

fn is_param(input: String) -> bool {
    return input.starts_with("-");
}

struct Args {
    recurse: bool,
    verbose: bool,
    interactive: bool,
    force: bool
}

fn parse_params(params: String) -> Result<Args,()> {
    if !params.starts_with("-") {
        Err::<Args, ()>(());
    }
    let cleanParams = params.split('-').nth(1);
    println!("Clean Params {:#?}", cleanParams.unwrap());
    let parsed = Args {
        recurse: false,
        verbose: false,
        interactive: false,
        force: false
    };
    // for singleparam in cleanParams {
    //     if(singleparam == "r") {
    //         parsed.recurse == true;
    //     } else if (singleparam == "v") {
    //         parsed.verbose == true;
    //     } else if (singleparam == "i") {
    //         parsed.interactive == true;
    //     } else if (singleparam == "f") {
    //         parsed.interactive == true;
    //     }
    // }

    Ok(parsed)
}