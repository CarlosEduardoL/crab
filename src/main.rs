mod args;
mod reader;
mod writer;
mod errors;

use std::any::Any;
use std::thread;
use clap::Parser;
use crossbeam::{Receiver, Sender, unbounded};
use crate::args::CrabArgs;
use crate::reader::InputSource::{File, Stdin};
use crate::reader::{InputSource, Reader};
use crate::writer::Writer;

fn main() -> Result<(), Box<dyn Any + Send>> {
    let mut args = CrabArgs::parse();
    fix_args(&mut args);
    let sources: Vec<InputSource> = get_sources(&args);
    let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = unbounded();
    let mut reader = Reader::new(sender);
    let mut writer = Writer::new(args, receiver);
    let children = vec![
        thread::spawn(move || {
            writer.write()
        }),
        thread::spawn(move || {
            for source in sources {
                reader.read_source(source);
            }
        }),
    ];

    for child in children {
        child.join()?;
    }
    Ok(())
}

fn get_sources(args: &CrabArgs) -> Vec<InputSource> {
    if args.files.is_empty() { return vec![Stdin]; }
    let mut res: Vec<InputSource> = Vec::with_capacity(args.files.len());
    for file in &args.files {
        res.push(if file == "-" { Stdin } else { File(file.to_string()) })
    }
    res
}

fn fix_args(args: &mut CrabArgs) {
    args.show_non_printing |= args.ev || args.show_all || args.t;
    args.show_ends |= args.ev || args.show_all;
    args.show_tabs |= args.show_all || args.t;
    args.number_lines |= args.number_non_blank;
}