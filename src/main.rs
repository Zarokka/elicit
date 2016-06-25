mod argparse;
mod filesearch;
use filesearch::HitIterator;
use std::iter::Iterator;
use std::env;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let argstore = match argparse::parseargs(&args) {
        Some(v) => v,
        None => {
            println!("Usage: elicit [OPTIONS] [DIRECTORY] PATTERN \n\
                      Elicit (or find) files recursively from the DIRECTORY that contain the \
                      given PATTERN\n(or match the given PATTERN as regex if option -r or --regex \
                      is used).\n\n\
                      Options: \n  \
                        -r, --regex\tpattern is a regular expression (NOT YET IMPLEMENTED) \n      \
                            --help\tshow this message");
            return;
        }
    };

    let hititer: HitIterator = filesearch::find(argstore).unwrap();

    // for hit in &hititer gave trait bounds not satisfied error, why?
    for hit in hititer {
        println!("{:?}", hit);
    }
}
