use argparse::ArgumentStore;
use std::fs::{self, ReadDir, DirEntry};
use std::path::PathBuf;
use std::iter::Iterator;
use std::io::{self, Result, Error, Write};


pub struct HitIterator {
    pattern: String,
    readdirs: Vec<ReadDir>,
}

impl HitIterator {
    fn next_result(&mut self) -> Result<Option<PathBuf>> {

        if self.readdirs.len() == 0 {
            return Ok(None);
        }
        // get_mut should be save, because of the above check
        match self.readdirs.get_mut(0).unwrap().next() {
            Some(v) => {
                let entry: DirEntry = try!(v);
                if try!(entry.file_type()).is_dir() {
                    let nextreaddir = try!(fs::read_dir(entry.path()));
                    self.readdirs.push(nextreaddir);
                    return self.next_result();
                }

                let filename = entry.file_name();
                // TODO don't panic if the filename cannot be convertet to a string
                let filename: String = filename.into_string().unwrap();
                if filename.as_str().contains(self.pattern.as_str()) {
                    return Ok(Some(entry.path()));
                }

                return self.next_result();
            }
            None => {
                self.readdirs.remove(0);
                return self.next_result();
            }

        }
    }
}

fn print_err(err: &Error) {
    let r = writeln!(&mut io::stderr(), "{}", err);
    r.expect("Failed to write error to stderr!");
}

impl Iterator for HitIterator {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        match self.next_result() {
            Ok(res) => {
                return res;
            }
            Err(e) => {
                print_err(&e);
                return self.next();
            }
        };
    }
}

pub fn find(args: ArgumentStore) -> Result<HitIterator> {

    // TODO require pattern
    let dir: PathBuf = if args.dir.is_some() {
        PathBuf::from(args.dir.unwrap())
    } else {
        // TODO platform independent relative path
        PathBuf::from("./")
        // this would give the full current path
        // try![env::current_dir()]
    };

    Ok(HitIterator {
        pattern: args.pattern.unwrap(),
        readdirs: vec![try!(fs::read_dir(dir))],
    })
}

// fn search(args: &ArgumentStore, callback: &Fn(&str)) {}

#[cfg(test)]
mod filesearchtest {

    use argparse::ArgumentStore;
    use std::path::PathBuf;
    use std::env;
    use std::fs::remove_dir_all;
    use std::fs::create_dir;
    use std::fs::File;
    use filesearch::{self, HitIterator};

    fn createfile(path: &PathBuf, filename: &str) {
        let mut path = path.clone();
        path.push(filename);
        File::create(path).expect("Can not create file in temporary testing directory");
    }

    fn preparetestfiles() {
        let mut tempdir: PathBuf = env::temp_dir();
        tempdir.push("elicit_temporary_test_dir");
        if tempdir.exists() {
            remove_dir_all(&tempdir).expect("Can not remove old testing directory.");
        }
        create_dir(&tempdir).expect("Can not create temporary testing directory");

        createfile(&tempdir, "hello");
        createfile(&tempdir, "whatuphelloworld");
        createfile(&tempdir, "no_he_ll_o_in_this_file_name");

        tempdir.push("2nd level");
        create_dir(&tempdir).expect("Can not create 2nd level temporary testing directory");

        createfile(&tempdir, "2nd level hello");
        createfile(&tempdir, "another none match");
        createfile(&tempdir, "xyz");

        tempdir.push("3rd level");
        create_dir(&tempdir).expect("Can not create 3rd level temporary testing directory");

        createfile(&tempdir, "hello from the 3rd level");
        createfile(&tempdir, "another none match");
    }

    #[test]
    fn filesearch_by_pattern() {
        preparetestfiles();

        let mut tempdir: PathBuf = env::temp_dir();
        tempdir.push("elicit_temporary_test_dir");
        assert!(tempdir.exists());
        let dirstr = tempdir.to_str().unwrap();
        let args = ArgumentStore {
            pattern: Some("hello".to_string()),
            dir: Some(dirstr.to_string()),
            isregex: false,
        };

        let hititer = filesearch::find(args).unwrap();

        let mut results: Vec<PathBuf> = hititer.collect();
        assert_eq!(results.len(), 4);
        let lvl2results = results.split_off(2);
        results.sort();

        // TODO use platform independent path separator
        assert_eq!(results[0].to_str().unwrap(), format!("{}/hello", dirstr));
        assert_eq!(results[1].to_str().unwrap(),
                   format!("{}/whatuphelloworld", dirstr));
        assert_eq!(lvl2results[0].to_str().unwrap(),
                   format!("{}/2nd level/2nd level hello", dirstr));
        assert_eq!(lvl2results[1].to_str().unwrap(),
                   format!("{}/2nd level/3rd level/hello from the 3rd level", dirstr));
    }
}
