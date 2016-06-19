use argparse::ArgumentStore;
use std::fs::{self, ReadDir};
use std::path::PathBuf;
use std::iter::Iterator;
use std::env;
use std::io::{self, Result, Error, Write};


pub struct HitIterator<'a> {
    args: ArgumentStore<'a>,
    readdirs: Vec<ReadDir>,
}

// impl<'a> HitIterator<'a> {
//     fn new(args: ArgumentStore) -> HitIterator {
//     }
// }

fn print_err(err: &Error) {
    let r = writeln!(&mut io::stderr(), "{}", err);
    r.expect("Failed to write error to stderr!");
}

impl<'a> Iterator for HitIterator<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        if self.readdirs.len() == 0 {
            return None;
        }
        // TODO macro the error stuff, maybe allow the user to specify an error function
        // add the path to error where it occured (currently only "Permission denied (os error 13)")
        match self.readdirs.get_mut(0).unwrap().next() {
            Some(v) => {
                let entry = match v {
                    Ok(v) => v,
                    Err(e) => {
                        print_err(&e);
                        return self.next();
                    }
                };
                match entry.file_type() {
                    Ok(v) => {
                        if v.is_dir() {
                            match fs::read_dir(entry.path()) {
                                Ok(v) => {
                                    self.readdirs.push(v);
                                }
                                Err(e) => {
                                    print_err(&e);
                                }
                            }
                            return self.next();
                        }
                    }
                    Err(e) => {
                        print_err(&e);
                        return self.next();
                    }
                }

                let filename = entry.file_name();
                // TODO don't panic
                if filename.to_str().unwrap().contains(self.args.pattern.unwrap()) {
                    return Some(entry.path());
                }

                return self.next();
            }
            None => {
                self.readdirs.remove(0);
                return self.next();
            }

        }
    }
}

pub fn find(args: ArgumentStore) -> Result<HitIterator> {

    let dir: PathBuf = if args.dir.is_some() {
        PathBuf::from(args.dir.unwrap())
    } else {
        //TODO platform independent relative path
        PathBuf::from("./")
        // this would give the full current path
        //try![env::current_dir()]
    };

    Ok(HitIterator {
        args: args,
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
            pattern: Some("hello"),
            dir: Some(dirstr),
            isregex: false,
        };

        let hititer = filesearch::find(args).unwrap();

        let mut results: Vec<PathBuf> = hititer.collect();
        assert_eq!(results.len(), 4);
        let lvl2results = results.split_off(2);
        results.sort();

        //TODO use platform independent path separator
        assert_eq!(results[0].to_str().unwrap(), format!("{}/hello", dirstr));
        assert_eq!(results[1].to_str().unwrap(),
                   format!("{}/whatuphelloworld", dirstr));
        assert_eq!(lvl2results[0].to_str().unwrap(),
                   format!("{}/2nd level/2nd level hello", dirstr));
        assert_eq!(lvl2results[1].to_str().unwrap(),
                   format!("{}/2nd level/3rd level/hello from the 3rd level", dirstr));
    }
}
