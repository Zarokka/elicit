use argparse::ArgumentStore;

fn search(args: ArgumentStore, callback: &Fn(&str)) {}

#[cfg(test)]
mod filesearchtest {

    use std::path::PathBuf;
    use std::env;
    use std::io;
    use std::fs::remove_dir_all;
    use std::fs::create_dir;
    use std::fs::File;

    fn createfile(mut path: PathBuf, filename: &str) -> io::Result<File> {
        path.push(filename);
        let file : File = try!(File::create(path));
        Ok(file)
    }

    fn preparetestfiles() {
        let mut tempdir: PathBuf = env::temp_dir();
        tempdir.push("elicit_temporary_test_dir");
        remove_dir_all(&tempdir);
        create_dir(&tempdir);

        createfile(tempdir, "hello");
    }

    #[test]
    fn filesearch_by_pattern() {
        preparetestfiles();

        let mut tempdir: PathBuf = env::temp_dir();
        tempdir.push("elicit_temporary_test_dir");

        assert!(tempdir.exists());
        tempdir.push("hello");
        assert!(tempdir.exists());

    }


}
