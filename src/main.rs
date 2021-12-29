mod myfile;
use myfile::Regex;

use std::path::PathBuf;
use structopt::StructOpt;

// const DIRS: [&str; 1] = ["./src"];
// const PATTERNS: [&str; 1] = ["*.rs"];

const BUFSIZE: usize = 1000;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust find", about = "A simple find utility, written in Rust.")]
pub struct Opt {
    #[structopt(short = "d", long = "dirs", parse(from_os_str), help = "list of directories to search")]
    dirs: Vec<PathBuf>,

    #[structopt(short = "p", long = "patterns", parse(from_str = parse_regex_with_warn), help = "list of patterns to use")]
    patterns: Vec<Option<Regex>>,

    #[structopt(short = "s", long = "size", help = "match files above size <size> bytes")]
    size: Option<usize>,

    #[structopt(short = "o", long = "output", help = "write results to file <output> intead of STDOUT")]
    output: Option<String>,
}

fn parse_regex_with_warn(pattern: &str) -> Option<Regex> {
    let re = Regex::new(pattern);
    if re.is_err() {
        println!("{}", re.unwrap_err());
        None
    } else {
        Some(re.unwrap())
    }
}

fn main() {
    // let mut dirs: Vec<PathBuf> = DIRS.iter().map(|d| d.into()).collect();
    let opt: Opt = StructOpt::from_args();
    let valid_patterns: Vec<Regex> = opt.patterns.iter().filter_map(|p| p.clone()).collect();
    if valid_patterns.is_empty() {
        println!("Input contained no valid patterns. Exiting.");
        return
    }

    let mut dirs = opt.dirs.clone();
    let mut files: Vec<myfile::MyFile> = Vec::with_capacity(BUFSIZE);
    while !dirs.is_empty() {
        let dir = dirs.pop().unwrap(); // safe (always non-empty)
        if let Err(e) = myfile::walk(&dir, &mut dirs, &valid_patterns, &mut files, &opt) {
            println!("{}", e);
        }
    }

    // flush remaining
    if let Err(e) = myfile::flush(&mut files, &opt.output) {
        println!("{}", e);
    };
}
