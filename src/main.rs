mod args;
mod errors;
mod reader;
#[cfg(test)]
mod test;

use crate::args::args;
use crate::reader::InputSource::{File, Stdin};
use crate::reader::{InputSource, Reader};

fn main() {
    let (files, args) = args();
    let sources: Vec<InputSource> = get_sources(files);
    let mut reader = Reader::new(args);
    for source in sources {
        reader.read_source(source);
    }
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
