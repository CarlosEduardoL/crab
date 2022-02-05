use io::stdin;
use std::fs::{File};
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use atty::Stream;
use crossbeam::{Sender};
use CrabError::OpenError;
use crate::errors::CrabError;
use crate::errors::CrabError::ReadError;

pub enum InputSource {
    File(String),
    Stdin,
}

pub struct Reader {
    sender: Sender<Vec<u8>>,
}

impl Reader {
    pub fn new(sender: Sender<Vec<u8>>) -> Self { Reader { sender } }

    fn open_file(_path: String) -> File {
        let path: PathBuf = PathBuf::from(&_path);
        match File::open(path) {
            Ok(file) => { file }
            Err(err) => { OpenError(_path, err).show_and_exit() }
        }
    }


    pub fn read_source(&mut self, source: InputSource) {
        match &source {
            InputSource::File(file) => { self.read(Self::open_file(file.to_string()), source) }
            InputSource::Stdin => { if atty::isnt(Stream::Stdin) { self.read(stdin().lock(), source) } }
        }
    }

    fn read<T: Read>(&mut self, inner: T, source: InputSource) {
        let mut reader: BufReader<T> = BufReader::new(inner);
        loop {
            let mut line: Vec<u8> = Vec::new();
            let result: std::io::Result<usize> = reader.read_until(b'\n', &mut line);
            match result {
                Ok(0) => { return; }
                Ok(_n) => {}
                Err(err) => { ReadError(source, err).show_and_exit() }
            }
            if self.sender.send(line).is_err() { return; }
        }
    }
}