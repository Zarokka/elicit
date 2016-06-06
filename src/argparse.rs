
// the 'a is for lifetime management TODO learn more
#[derive(Default)]
pub struct ArgumentStore<'a> {
    pattern: Option<&'a str>,
    dir: Option<&'a str>,
    isregex: bool,
}

// default values could be set like this
// impl Default for ArgumentStore {
//     fn default() -> ArgumentStore {
//         ArgumentStore { pattern: None(), aFooFoo: 6 }
//     }
// }

pub fn parseargs(args: &Vec<String>) -> ArgumentStore {
    let mut argstore: ArgumentStore = Default::default();

    // for (i, arg) in args.iter().enumerate() {
    for arg in args {
        match arg.as_ref() {
            "--regex" | "-r" => argstore.isregex = true,
            _ => {
                if argstore.pattern.is_none() {
                    argstore.pattern = Some(arg);
                } else if argstore.dir.is_none() {
                    argstore.dir = Some(arg);
                }
            }
        }

    }

    argstore
}

#[cfg(test)]
mod argparsetest {

    use argparse::{ArgumentStore, parseargs};

    #[test]
    fn argparse_onearg_test() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("hello".to_string());
        let args: ArgumentStore = parseargs(&v);

        assert_eq!(args.pattern.unwrap(), "hello");
        assert!(args.dir.is_none());
        assert!(!args.isregex);
    }

    #[test]
    fn argparse_twoargs_test() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("hello".to_string());
        v.push("~/".to_string());
        let args: ArgumentStore = parseargs(&v);

        assert_eq!(args.pattern.unwrap(), "hello");
        assert_eq!(args.dir.unwrap(), "~/");
        assert!(!args.isregex);
    }

    #[test]
    fn argparse_threargs1_test() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("-r".to_string());
        v.push("hello".to_string());
        v.push("~/".to_string());
        let args: ArgumentStore = parseargs(&v);

        assert_eq!(args.pattern.unwrap(), "hello");
        assert_eq!(args.dir.unwrap(), "~/");
        assert!(args.isregex);
    }

    #[test]
    fn argparse_threargs2_test() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("hello".to_string());
        v.push("~/".to_string());
        v.push("--regex".to_string());
        let args: ArgumentStore = parseargs(&v);

        assert_eq!(args.pattern.unwrap(), "hello");
        assert_eq!(args.dir.unwrap(), "~/");
        assert!(args.isregex);
    }
}
