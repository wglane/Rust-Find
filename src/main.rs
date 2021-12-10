mod myfile;

use std::path::PathBuf;


const DIRS: [&str; 1] = ["./src"];
// const PATTERNS: [&str; 1] = ["*.rs"];

const BUFSIZE: usize = 1000;

fn main() {
    let mut dirs: Vec<PathBuf> = DIRS.iter().map(|d| d.into()).collect();
    let mut files: Vec<myfile::MyFile> = Vec::with_capacity(BUFSIZE);

    while !dirs.is_empty() {
        let dir = dirs.pop().unwrap(); // safe
        if let Err(e) = myfile::walk(&dir, &mut dirs, &mut files) {
            println!("{}", e);
        }
    }

    // flush remaining
    myfile::flush(&mut files);
}
