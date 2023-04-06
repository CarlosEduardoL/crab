mod args;
mod errors;
mod reader;
mod mapping;
#[cfg(test)]
mod test;

use crate::args::ARGS;
use crate::reader::InputSource::{File, Stdin};
use crate::reader::{InputSource, Reader};
use std::process::exit;

fn main() {
    let sources: Vec<InputSource> = get_sources(&ARGS.files);
    let mut reader = Reader::new();
    let mut exit_code = 0u8;
    for source in sources {
        exit_code |= reader.read_source(source);
    }
    exit(exit_code as i32)
}

fn get_sources(files: &Vec<String>) -> Vec<InputSource> {
    if files.is_empty() {
        return vec![Stdin];
    }
    let mut res: Vec<InputSource> = Vec::with_capacity(files.len());
    for file in files {
        res.push(if file == "-" { Stdin } else { File(file.clone()) })
    }
    res
}
