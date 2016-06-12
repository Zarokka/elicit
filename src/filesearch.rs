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
        let mut readdir = self.readdirs.pop().unwrap();
        
        match readdir.next() {
            Some(v) => {
                // TODO check if dir
                let entrypath = v.unwrap().file_name();
                self.readdirs.push(readdir);
                return Some(entrypath);
            },
            None => {
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


        assert_eq!(results.len(), 2);
        assert_eq!(results[0], OsString::from("hello"));
        assert_eq!(results[1], OsString::from("whatuphelloworld"));
    }


}
