mod args;
mod reader;

use clap::Parser;
use crate::args::CrabArgs;
use crate::reader::Reader;

fn main() {
    let reader = &mut Reader::new(args::CrabArgs::parse());
    let files = reader.get_files();
    if files.is_empty() { reader.read_stdin() }
    for file in &files {
        if file.to_str().unwrap() == "-" { reader.read_stdin() } else { reader.read_file(file) };
    }
}

