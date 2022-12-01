use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub struct ReadResult {
    pub content: String,
    pub position: u64,
}

pub enum LogFileDescriptor<'a> {
    Path(Box<str>),
    File(&'a File),
}

fn read_contents_file(mut file: &File) -> Result<ReadResult, io::Error> {
    let mut buffer = String::new();
    let size = file.read_to_string(&mut buffer)? as u64;
    Ok(ReadResult {
        content: buffer,
        position: size,
    })
}

fn read_contents_path(path: &str) -> Result<ReadResult, io::Error> {
    let mut file = File::open(path)?;
    read_contents_file(&file)
}

pub fn read_contents(descriptor: LogFileDescriptor) -> Result<ReadResult, io::Error> {
    match descriptor {
        LogFileDescriptor::Path(path) => read_contents_path(&path),
        LogFileDescriptor::File(file) => read_contents_file(&file),
    }
}

pub fn read_contents_from(
    descriptor: LogFileDescriptor,
    position: u64,
) -> Result<ReadResult, io::Error> {
    match descriptor {
        LogFileDescriptor::Path(path) => read_contents_from_path(&path, position),
        LogFileDescriptor::File(file) => read_contents_from_file(&file, position),
    }
}

fn read_contents_from_path(path: &str, position: u64) -> Result<ReadResult, io::Error> {
    let mut file = File::open(path)?;
    read_contents_from_file(&file, position)
}

fn read_contents_from_file(mut file: &File, position: u64) -> Result<ReadResult, io::Error> {
    let mut buffer = String::new();
    file.seek(SeekFrom::Start(position)).unwrap();
    file.read_to_string(&mut buffer)?;
    let size = file.metadata().unwrap().len();
    Ok(ReadResult {
        content: buffer,
        position: size,
    })
}
