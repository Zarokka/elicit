mod argparse;
use std::io::{self, Write};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;
use std::env;

fn list_files(dir: PathBuf) -> Result<(), io::Error> {
    for entry in try!(fs::read_dir(dir)) {
        let file = try!(entry);
        println!("{:?}", file.path());
        // try!(io::stdout().write(file.path().to_str().unwrap().as_bytes()));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir: PathBuf = env::current_dir().unwrap();
    match args.len() {
        1 => {
            list_files(dir);
        }
        2 => {
            list_files(PathBuf::from(&args[1]));
        }
        _ => {
            println!("Wrong usage");
        }
    }
}
