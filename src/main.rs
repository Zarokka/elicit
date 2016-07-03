extern crate walkdir;
mod argparse;
use walkdir::WalkDir;
use std::env;
use std::io::{self, Write};

pub fn matches(patterns: &Vec<String>, filename: &str) -> bool {
    let mut index: usize = 0;
    for pattern in patterns {
        if index >= filename.len() {
            return false;
        }
        match filename[index..].find(pattern) {
            Some(i) => index += i + pattern.len(),
            None => return false,
        }
    }
    return true;
}

fn print_usage() {
    println!("Usage: elicit [OPTIONS] [DIRECTORY] PATTERN \n\
              Elicit (or find) files recursively from the DIRECTORY that contain the \
              given PATTERN. It does not follow links.\n\n\
              Options: \n    \
              --help\tshow this message");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let argstore = match argparse::parseargs(&args) {
        Some(v) => v,
        None => {
            print_usage();
            return;
        }
    };

    let patterns: Vec<String> =
        argstore.pattern.unwrap().split("*").map(|s| s.to_string()).collect();


    let dir = if argstore.dir.is_some() {
        argstore.dir.unwrap()
    } else {
        let mut s = ".".to_string();
        s.push(std::path::MAIN_SEPARATOR);
        s
        // this would give the full current path
        // try![env::current_dir()]
    };
    let walker = WalkDir::new(dir).into_iter();

    // for entry in walker.filter_entry(|e| matches(&patterns, e.file_name().to_str().unwrap())) {
    for entry in walker {
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_str();
                if file_name.is_none() {
                    let r = writeln!(&mut io::stderr(),
                                     "File name not a valid UTF string: {}",
                                     e.path().display());
                    r.expect("Failed to write error to stderr!");
                    continue;
                }
                if matches(&patterns, file_name.unwrap()) {
                    println!("{}", e.path().display());
                }
            }
            Err(err) => {
                let r = writeln!(&mut io::stderr(), "{}", err);
                r.expect("Failed to write error to stderr!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_simple() {
        let patterns: Vec<String> = vec!["hello".to_string()];

        assert!(matches(&patterns, "helloatstart"));
        assert!(matches(&patterns, "in_the_hello_middle"));
        assert!(matches(&patterns, "at the end hello"));

        assert!(!matches(&patterns, "hel loatstart"));
        assert!(!matches(&patterns, "in_the_hell_middle"));
        assert!(!matches(&patterns, "at the end hollo"));
    }

    #[test]
    fn matches_multi() {
        let patterns: Vec<String> = vec!["hello".to_string(), "world".to_string()];

        assert!(matches(&patterns, "hello followed by world somewhere"));
        assert!(matches(&patterns, "in_the_helloworld_middle"));
        assert!(matches(&patterns, "blabla hello blabla worldblabla"));

        assert!(!matches(&patterns, "world hello"));
        assert!(!matches(&patterns, "in_the_hello orld_middle"));
        assert!(!matches(&patterns, "blabla hell blabla worldblabla"));
    }

    #[test]
    fn matches_multi_index() {
        let patterns: Vec<String> = vec!["l".to_string(), "l".to_string(), "l".to_string()];

        assert!(matches(&patterns, "lll"));
        assert!(!matches(&patterns, "ll"));
    }

    #[test]
    fn matches_multi_index_unicode() {
        let patterns: Vec<String> = vec!["他".to_string(), "他".to_string(), "他".to_string()];

        assert!(matches(&patterns, "他他他"));
        assert!(!matches(&patterns, "他他"));
    }
}
