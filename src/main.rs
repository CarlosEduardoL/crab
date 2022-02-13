mod args;
mod errors;
mod reader;
#[cfg(test)]
mod test;

use crate::args::args;
use crate::reader::InputSource::{File, Stdin};
use crate::reader::{InputSource, Reader};
use std::process::exit;

fn main() {
    let (files, args) = args();
    let sources: Vec<InputSource> = get_sources(files);
    let mut reader = Reader::new(args);
    let mut exit_code = 0u8;
    for source in sources {
        exit_code |= reader.read_source(source);
    }
    exit(exit_code as i32)
}

fn get_sources(files: Vec<String>) -> Vec<InputSource> {
    if files.is_empty() {
        return vec![Stdin];
    }
    let mut res: Vec<InputSource> = Vec::with_capacity(files.len());
    for file in files {
        res.push(if file == "-" { Stdin } else { File(file) })
    }
    res
}
