use argparse::ArgumentStore;

pub struct HitIterator<'a> {
    args: ArgumentStore<'a>,
}

impl<'a> HitIterator<'a> {

    fn new(args: ArgumentStore) -> HitIterator {
        HitIterator {args: args}
    }
}

impl<'a> Iterator for HitIterator<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        //Some("hello")
        None
    }
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
    use filesearch::HitIterator;
    //use filesearch::search;

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
        tempdir.push("hello");
        assert!(tempdir.exists());


        let args = ArgumentStore{pattern:Some("hello"), .. Default::default()};

        let mut hititer = HitIterator::new(args);

        let mut results : Vec<&str> = hititer.collect();


        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "hello");
        assert_eq!(results[1], "whatuphelloworld");
    }


}
