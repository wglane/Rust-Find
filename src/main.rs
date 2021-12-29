mod myfile;
use myfile::Regex;

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

    // #[structopt(short = "o", long = "output", help = "write results to file <output> intead of STDOUT")]
}

fn parse_all_with_warn(patterns: &Vec<String>) -> Vec<Regex> {
    let mut parsed: Vec<Regex> = vec![];
    for pattern in patterns {
        let re = Regex::new(pattern);
        if re.is_err() {
            println!("{}", re.unwrap_err());
            continue;
        }
        parsed.push(re.unwrap()); 
    }
    parsed
}

fn main() {
    // let mut dirs: Vec<PathBuf> = DIRS.iter().map(|d| d.into()).collect();
    let opt: Opt = StructOpt::from_args();
    let mut dirs = opt.dirs.clone();
    let mut files: Vec<myfile::MyFile> = Vec::with_capacity(BUFSIZE);
    let patterns = parse_all_with_warn(&opt.patterns);
    if patterns.is_empty() {
        println!("Input contained no valid patterns. Exiting.");
        return
    }

    while !dirs.is_empty() {
        let dir = dirs.pop().unwrap(); // safe (always non-empty)
        if let Err(e) = myfile::walk(&dir, &mut dirs, &patterns, &mut files) {
            println!("{}", e);
        }
    }

    // flush remaining
    myfile::flush(&mut files);
}
