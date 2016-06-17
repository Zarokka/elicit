use argparse::ArgumentStore;
use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;
use std::env;
use std::io::Result;
use std::ffi::OsString;

pub struct HitIterator<'a> {
    args: ArgumentStore<'a>,
    readdirs: Vec<ReadDir>,
}

// impl<'a> HitIterator<'a> {
//     fn new(args: ArgumentStore) -> HitIterator {
//     }
// }

impl<'a> Iterator for HitIterator<'a> {

    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        if self.readdirs.len() == 0 {
            return None;
        }
        //let mut readdir = self.readdirs.get_mut(0).unwrap();

        match self.readdirs.get_mut(0).unwrap().next() {
            Some(v) => {
                // TODO dont panic, always retrun None on Error?
                // what if permission to one dir fails, but rest would work?
                let entry = v.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    //TODO lower levels should have the parent path in them (until search root levelt)
                    self.readdirs.push(fs::read_dir(entry.path()).unwrap());
                    return self.next();
                }

                let filename = entry.file_name();
                if filename.to_str().unwrap().contains(self.args.pattern.unwrap()) {
                    return Some(filename);
                }

               return self.next(); 
            },
            None => {
                self.readdirs.remove(0);
                return self.next();
            }

        }

        //Some("hello")
        None
    }
}

fn find(args: ArgumentStore) -> Result<HitIterator> {

    let dir: PathBuf = if args.dir.is_some() {
        PathBuf::from(args.dir.unwrap())
    } else {
        env::current_dir().unwrap()  
    };

    Ok(HitIterator {args: args, readdirs: vec![try!(fs::read_dir(dir))]})
}

//fn search(args: &ArgumentStore, callback: &Fn(&str)) {}

#[cfg(test)]
mod filesearchtest {

    use argparse::ArgumentStore;
    use std::path::PathBuf;
    use std::env;
    use std::io;
    use std::fs::remove_dir_all;
    use std::fs::create_dir;
    use std::fs::File;
    use filesearch::{self, HitIterator};
    //use filesearch::search;
    use std::ffi::OsString;

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

        let args = ArgumentStore{pattern:Some("hello"), dir: tempdir.to_str(), isregex: false};

        let mut hititer = filesearch::find(args).unwrap();

        let mut results : Vec<OsString> = hititer.collect();
        assert_eq!(results.len(), 4);
        let mut lvl2results = results.split_off(2);
        results.sort();

        assert_eq!(results[0], OsString::from("hello"));
        assert_eq!(results[1], OsString::from("whatuphelloworld"));
        assert_eq!(lvl2results[0], OsString::from("2nd level hello"));
        assert_eq!(lvl2results[1], OsString::from("hello from the 3rd level"));
    }


}
