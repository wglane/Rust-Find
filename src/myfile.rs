use std::fs::{read_dir, DirEntry, OpenOptions};
use std::io::{Error, ErrorKind, Result, Write, BufWriter};
use std::path::PathBuf;

use crate::Opt;

pub use regex::Regex;

#[derive(Debug)]
pub struct MyFile {
    pub name: String,
    in_dir: PathBuf,
    size_bytes: u64,
    // depth: usize, // TODO:
}

impl MyFile {
    pub fn from(entry: &DirEntry) -> Result<MyFile> {
        let name = entry.file_name().to_string_lossy().into_owned();
        let in_dir = entry
            .path()
            .parent()
            .ok_or(Error::from(ErrorKind::NotFound))?
            .to_path_buf();
        let md = entry.metadata()?;
        let size_bytes = md.len();

        Ok(MyFile {
            name,
            in_dir,
            size_bytes,
        })
    }
}

pub fn walk(dir: &PathBuf, dirs: &mut Vec<PathBuf>, patterns: &Vec<Regex>, files: &mut Vec<MyFile>, options: &Opt) -> Result<()> {
    // returns an io::Result for any issues that might come up during the walk
    // suggested here: [Stack Overflow](https://stackoverflow.com/a/49785300/4577129)
    let entries = read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let md = entry.metadata()?;
        if md.is_dir() {
            dirs.push(entry.path());
        } else if md.is_file() {
            let file = MyFile::from(&entry)?;
            if files.len() >= files.capacity() {
                dbg!("File buffer capacity ({}) exceeded! Flushing to STDOUT...", files.capacity());
                flush(files, &options.output)?;
            }
            if is_match(&file, &patterns, &options) {
                files.push(file);
            }
        }
    }
    // flush remaining
    flush(files, &options.output)?;
    Ok(())
}

fn is_match(file: &MyFile, patterns: &Vec<Regex>, options: &Opt) -> bool {
    if let Some(min_size) = options.size {
        if file.size_bytes < min_size {
            return false
        }
    }
    for pattern in patterns {
        if !pattern.is_match(&file.name) {
            return false
        }
    }
    true
}

pub fn flush(files: &mut Vec<MyFile>, output: &Option<String>) -> Result<()> {
    if let Some(outfile) = output {
        let f = OpenOptions::new().create(true).append(true).open(outfile)?;
        let mut f = BufWriter::new(f);
        while !files.is_empty() {
            let file = files.pop().unwrap();
            write![f, "{}\n", file.name]?;
        }
    }
    else {
        while !files.is_empty() {
            println!("{:?}", files.pop().unwrap());
        }
    }
    Ok(())
}
