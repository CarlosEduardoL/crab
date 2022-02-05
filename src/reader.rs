use std::fs::{File};
use std::io;
use std::io::{BufRead, BufReader, Read, Stdin};
use std::path::PathBuf;
use std::process::exit;
use atty::Stream;
use crossbeam::{Sender};

pub enum InputSource {
    File(PathBuf),
    Stdin,
}

pub struct Reader {
    sender: Sender<Vec<u8>>,
}

impl Reader {
    pub fn new(sender: Sender<Vec<u8>>) -> Self { Reader { sender } }

    fn read_file(&mut self, path: &PathBuf) {
        if !path.exists() {
            eprintln!("{} must exist", path.display());
            exit(1)
        }
        if !path.is_file() {
            eprintln!("{} must be a file", path.display());
            exit(2)
        }
        match File::open(path) {
            Ok(file) => {
                self.read(file)
            }
            Err(err) => {
                eprintln!("Unable to open file {}: {}", path.display(), err.to_string());
                exit(err.raw_os_error().unwrap_or(4))
            }
        }
    }

    fn read_stdin(&mut self) {
        if atty::isnt(Stream::Stdin) {
            let stdin: Stdin = io::stdin();
            self.read(stdin.lock())
        }
    }

    pub fn read_source(&mut self, source: InputSource) {
        match source {
            InputSource::File(file) => { self.read_file(&file) }
            InputSource::Stdin => { self.read_stdin() }
        }
    }

    fn read<T: Read>(&mut self, inner: T) {
        let mut reader: BufReader<T> = BufReader::new(inner);
        loop {
            let mut line: Vec<u8> = Vec::new();
            let result = reader.read_until(b'\n', &mut line);
            match result {
                Ok(0) => { return; }
                Ok(_n) => {}
                Err(err) => {
                    eprintln!("Unable to read line: {}", err.to_string());
                    exit(err.raw_os_error().unwrap_or(3))
                }
            }
            match self.sender.send(line) {
                Ok(_) => {}
                Err(_) => { return; }
            }
        }
    }
}