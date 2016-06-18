mod argparse;
mod filesearch;
use filesearch::HitIterator;
use std::iter::Iterator;
use std::env;


fn main() {
    let args : Vec<String> = env::args().skip(1).collect();
    let argstore = argparse::parseargs(&args);

    let hititer : HitIterator = filesearch::find(argstore).unwrap();
    //let mut results : Vec<PathBuf> = hititer.collect();

    // loop {
    //     match hititer.next() {
    //         Some(hit) => { println!("{:?}", hit);},
    //         None => break,
    //     }
    // }
    // for hit in &hititer gave trait bounds not satisfied error, why?
    for hit in hititer {
       println!("{:?}", hit);
    }
}
