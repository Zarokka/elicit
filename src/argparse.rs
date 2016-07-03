
#[derive(Default)]
pub struct ArgumentStore {
    pub pattern: Option<String>,
    pub dir: Option<String>,
    pub isregex: bool,
}

// default values could be set like this
// impl Default for ArgumentStore {
//     fn default() -> ArgumentStore {
//         ArgumentStore { pattern: None(), aFooFoo: 6 }
//     }
// }

pub fn parseargs(args: &Vec<String>) -> Option<ArgumentStore> {

    if args.len() > 3 || args.is_empty() {
        return None;
    }
    let mut argstore: ArgumentStore = Default::default();

    for arg in args {
        match arg.as_ref() {
            "--help" => {
                return None;
            }
            "--regex" | "-r" => argstore.isregex = true,
            _ => {
                if argstore.pattern.is_none() {
                    argstore.pattern = Some(arg.clone());
                } else if argstore.dir.is_none() {
                    argstore.dir = argstore.pattern;
                    argstore.pattern = Some(arg.clone());
                }
            }
        }
    }

    Some(argstore)
}

#[cfg(test)]
mod tests {

    use argparse::{ArgumentStore, parseargs};

    #[test]
    fn one_arg() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("hello".to_string());
        let args: ArgumentStore = parseargs(&v).unwrap();

        assert_eq!(args.pattern.unwrap(), "hello".to_string());
        assert!(args.dir.is_none());
        assert!(!args.isregex);
    }

    #[test]
    fn two_args() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("~/".to_string());
        v.push("hello".to_string());
        let args: ArgumentStore = parseargs(&v).unwrap();

        assert_eq!(args.pattern.unwrap(), "hello".to_string());
        assert_eq!(args.dir.unwrap(), "~/".to_string());
        assert!(!args.isregex);
    }

    #[test]
    fn three_args() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("-r".to_string());
        v.push("~/".to_string());
        v.push("hello".to_string());
        let args: ArgumentStore = parseargs(&v).unwrap();

        assert_eq!(args.pattern.unwrap(), "hello".to_string());
        assert_eq!(args.dir.unwrap(), "~/".to_string());
        assert!(args.isregex);
    }

    #[test]
    fn three_args_2() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("./somepath".to_string());
        v.push("whatup".to_string());
        v.push("--regex".to_string());
        let args: ArgumentStore = parseargs(&v).unwrap();

        assert_eq!(args.pattern.unwrap(), "whatup".to_string());
        assert_eq!(args.dir.unwrap(), "./somepath".to_string());
        assert!(args.isregex);
    }

    #[test]
    fn no_args() {
        let v: Vec<String> = Vec::<String>::new();
        assert!(parseargs(&v).is_none());
    }

    #[test]
    fn too_many_args() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("./somepath".to_string());
        v.push("whatup".to_string());
        v.push("--regex".to_string());
        v.push("-xyz".to_string());
        assert!(parseargs(&v).is_none());
    }

    #[test]
    fn help() {
        let mut v: Vec<String> = Vec::<String>::new();
        v.push("--help".to_string());
        assert!(parseargs(&v).is_none());
    }
}
