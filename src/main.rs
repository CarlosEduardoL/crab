mod args;
mod reader;
mod errors;

use std::any::Any;
use crate::args::args;
use crate::reader::InputSource::{File, Stdin};
use crate::reader::{InputSource, Reader};

fn main() -> Result<(), Box<dyn Any + Send>> {
    let (files, args) = args();
    let sources: Vec<InputSource> = get_sources(files);
    let mut reader = Reader::new(args);
    for source in sources {
        reader.read_source(source);
    }
    Ok(())
}

fn get_sources(files: Vec<String>) -> Vec<InputSource> {
    if files.is_empty() { return vec![Stdin]; }
    let mut res: Vec<InputSource> = Vec::with_capacity(files.len());
    for file in files {
        res.push(if file == "-" { Stdin } else { File(file) })
    }
    res
}
