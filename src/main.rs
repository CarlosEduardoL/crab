mod args;
mod errors;
mod reader;
mod mapping;
mod writer;
#[cfg(test)]
mod test;

use std::io::{copy, stdout};
use crate::args::ARGS;
use crate::reader::{InputSource, Reader};
use std::process::exit;
use crate::writer::OutWriter;

/// The main function of the program.
fn main() {
    // Create a new reader with the input sources specified in the command-line arguments.
    let mut reader = Reader::new(get_sources(&ARGS.files));

    // Check if any of the output options are enabled.
    if !ARGS.show_non_printing && !ARGS.show_ends && !ARGS.show_tabs && !ARGS.number_lines && !ARGS.number_non_blank && !ARGS.squeeze_blank {
        // If none of the output options are enabled, just copy the input to standard output.
        copy(&mut reader, &mut stdout().lock()).unwrap();
    } else {
        // Otherwise, create a new OutWriter and copy the modified input to it.
        let mut writer = OutWriter::new();
        copy(&mut reader, &mut writer).unwrap();
    }

    // Exit the program with the exit code from the reader.
    exit(reader.exit_code as i32)
}


/// Returns a vector of input sources based on a list of file names.
///
/// If the list is empty, the vector will contain only `Stdin`.
/// If a file name is "-", it will be replaced with `Stdin`.
/// Otherwise, the file name will be used to create a `File` input source.
fn get_sources(files: &[String]) -> Vec<InputSource> {
    if files.is_empty() {
        // If the list of file names is empty, return a vector containing only `Stdin`.
        return vec![InputSource::Stdin];
    }

    // Use the `map` method to transform each file name into an input source.
    let res: Vec<InputSource> = files
        .iter()
        .map(|file| if file == "-" { InputSource::Stdin } else { InputSource::File(file.clone()) })
        .collect();

    // Return the result vector.
    res
}
