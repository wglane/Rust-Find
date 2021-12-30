mod myfile;
use myfile::Regex;

use std::path::PathBuf;
use structopt::StructOpt;

const BUFSIZE: usize = 1000;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust find", about = "A simple find utility, written in Rust.")]
pub struct Opt {
    #[structopt(
        short = "d",
        long = "dirs",
        parse(from_os_str),
        help = "list of directories to search"
    )]
    dirs: Vec<PathBuf>,

    #[structopt(
        short = "p", 
        long = "patterns", 
        parse(from_str = parse_regex_with_warn),
        help = "list of patterns to use"
    )]
    patterns: Vec<Option<Regex>>,

    #[structopt(
        short = "o",
        long = "output",
        help = "write (or append) results to file <output> intead of STDOUT"
    )]
    output: Option<String>,

    #[structopt(short = "s", long = "size", help = "match files above size <size> bytes")]
    size: Option<u64>,

    #[structopt(
        short = "r",
        long = "depth",
        help = "match files with recusion depth at most <depth>, zero-indexed"
    )]
    depth: Option<usize>,
}

pub struct DirLevel {
    path: PathBuf,
    level: usize,
}

impl DirLevel {
    pub fn from(path: PathBuf, level: usize) -> DirLevel {
        DirLevel { path, level }
    }

    pub fn root(path: PathBuf) -> DirLevel {
        DirLevel { path, level: 0 }
    }
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
        std::process::exit(1);
    }

    let mut dirs: Vec<DirLevel> = opt.dirs.iter().map(|d| DirLevel::root(d.clone())).collect();
    let mut files: Vec<myfile::MyFile> = Vec::with_capacity(BUFSIZE);
    while !dirs.is_empty() {
        let dir = dirs.pop().unwrap(); // safe (always non-empty)
        if let Some(max_level) = opt.depth {
            if dir.level > max_level {
                continue;
            }
        } else if let Err(e) = myfile::walk(&dir, &mut dirs, &valid_patterns, &mut files, &opt) {
            println!("{}", e);
        }
    }
}

#[test]
fn test_parse_regex_with_warn() {
    let good = r".*\.rs";
    let bad = r"*\.rs";
    assert!(parse_regex_with_warn(good).is_some());
    assert!(parse_regex_with_warn(bad).is_none());
}
