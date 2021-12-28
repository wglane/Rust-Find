mod myfile;

use std::path::PathBuf;
use structopt::StructOpt;

// const DIRS: [&str; 1] = ["./src"];
// const PATTERNS: [&str; 1] = ["*.rs"];

const BUFSIZE: usize = 1000;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust find", about = "A simple find utility, written in Rust.")]
struct Opt {
    #[structopt(short = "d", long = "dirs", parse(from_os_str), help = "list of directories to search")]
    dirs: Vec<PathBuf>,

    #[structopt(short = "p", long = "patterns", help = "list of patterns to use")]
    patterns: Vec<String>,

    #[structopt(short = "s", long = "size", help = "match files above size <size> bytes")]
    size: Option<usize>,
}

fn main() {
    // let mut dirs: Vec<PathBuf> = DIRS.iter().map(|d| d.into()).collect();
    let opt: Opt = StructOpt::from_args();
    let mut dirs = opt.dirs.clone();
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
