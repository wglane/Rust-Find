use std::fs::{DirEntry, OpenOptions, ReadDir};
use std::io::{BufWriter, Error, ErrorKind, Result, Write};
use std::path::PathBuf;

use crate::{DirLevel, Opt};

pub use regex::Regex;

#[derive(Debug)]
pub struct MyFile {
    pub name: String,
    in_dir: PathBuf,
    size_bytes: u64,
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

pub fn walk(
    dir: &DirLevel,
    dirs: &mut Vec<DirLevel>,
    patterns: &Vec<Regex>,
    files: &mut Vec<MyFile>,
    options: &Opt,
) -> Result<()> {
    let entries = read_dir(&dir)?;
    for entry in entries {
        let entry = entry?;
        let md = entry.metadata()?;
        if md.is_dir() {
            dirs.push(DirLevel::from(entry.path(), dir.level + 1));
        } else if md.is_file() {
            let file = MyFile::from(&entry)?;
            if files.len() >= files.capacity() {
                dbg!(
                    "File buffer capacity ({}) exceeded! Flushing to STDOUT...",
                    files.capacity()
                );
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

fn read_dir(dir: &DirLevel) -> Result<ReadDir> {
    let dir_open_error = Error::new(
        ErrorKind::Other,
        format!("Error: could not open {}", dir.path.to_string_lossy()),
    );
    match std::fs::read_dir(&dir.path) {
        Err(_) => Err(dir_open_error),
        Ok(dir) => Ok(dir),
    }
}

fn is_match(file: &MyFile, patterns: &Vec<Regex>, options: &Opt) -> bool {
    if let Some(min_size) = options.size {
        if file.size_bytes < min_size {
            return false;
        }
    }
    for pattern in patterns {
        if pattern.is_match(&file.name) {
            return true;
        }
    }
    false
}

pub fn flush(files: &mut Vec<MyFile>, output: &Option<String>) -> Result<()> {
    if let Some(outfile) = output {
        let f = OpenOptions::new().create(true).append(true).open(outfile)?;
        let mut f = BufWriter::new(f);
        while !files.is_empty() {
            let file = files.pop().unwrap();
            write![f, "{}\n", file.name]?;
        }
    } else {
        while !files.is_empty() {
            println!("{:?}", files.pop().unwrap());
        }
    }
    Ok(())
}

#[test]
fn test_is_match() {
    // TODO:
}
