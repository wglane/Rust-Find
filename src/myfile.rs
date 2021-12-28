use std::fs::{read_dir, DirEntry};
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

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

pub fn walk(dir: &PathBuf, dirs: &mut Vec<PathBuf>, files: &mut Vec<MyFile>) -> Result<()> {
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
                println!(
                    "File buffer capacity ({}) exceeded! Flushing to STDOUT...",
                    files.capacity()
                );
                flush(files);
            }
            files.push(file);
        }
    }
    Ok(())
}

// TODO:
// fn is_match(file: &MyFile) -> () {}

pub fn flush(files: &mut Vec<MyFile>) {
    while !files.is_empty() {
        println!("{:?}", files.pop().unwrap());
    }
}
