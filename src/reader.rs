use std::collections::HashMap;
use std::fs::{File};
use std::io;
use std::io::{BufRead, BufReader, Read, Stdin};
use std::path::PathBuf;
use std::process::exit;
use atty::Stream;
use crate::{char_mapping, CrabArgs};

pub struct Reader {
    args: CrabArgs,
    counter: usize,
    mapping: HashMap<u8, String>,
    last_line: String,
}

impl Reader {
    pub fn new(mut args: CrabArgs) -> Self {
        args.show_non_printing |= args.ev || args.show_all || args.t;
        args.show_ends |= args.ev || args.show_all;
        args.show_tabs |= args.show_all || args.t;
        args.number_lines |= args.number_non_blank;
        Reader {
            args,
            counter: 1,
            mapping: char_mapping(),
            last_line: String::new(),
        }
    }

    pub fn read_file(&mut self, file: &PathBuf) -> &mut Self {
        if !file.exists() {
            eprintln!("{} must exist", file.display());
            exit(1)
        }
        if !file.is_file() {
            eprintln!("{} must be a file", file.display());
            exit(2)
        }
        let file: File = File::open(file).expect("Unable to open file");
        return self.read(file);
    }

    pub fn read_stdin(&mut self) -> &mut Self {
        if atty::isnt(Stream::Stdin) {
            let stdin: Stdin = io::stdin();
            return self.read(stdin.lock());
        }
        return self;
    }

    fn read<T: Read>(&mut self, inner: T) -> &mut Self {
        let reader = BufReader::new(inner);
        let end = if self.args.show_ends { "$" } else { "" };
        let tab = if self.args.show_tabs { "^I" } else { "\t" };
        for line in reader.lines() {
            let line: String = line.expect("Unable to read line").replace("\t", tab);
            let line = if self.args.show_non_printing { self.show_non_printable(line.as_bytes()) } else { line };
            if self.args.squeeze_blank && self.last_line == line && line == "" { continue; }
            self.last_line = line;
            if self.args.number_lines {
                if self.args.number_non_blank && self.last_line == "" {
                    println!("{:>6}  {}{}", "", self.last_line, end);
                    continue;
                }
                println!("{:>6}\t{}{}", self.counter, self.last_line, end);
                self.counter += 1;
            } else { println!("{}{}", self.last_line, end) }
        }
        return self;
    }

    fn show_non_printable(&self, line: &[u8]) -> String {
        let mut result = String::new();
        for char in line {
            if self.mapping.contains_key(&char) {
                result.push_str(self.mapping.get(&char).unwrap())
            } else {
                result.push(*char as char)
            }
        }
        return result;
    }

    pub fn get_files(&mut self) -> (&mut Self, Vec<PathBuf>) {
        let mut files = Vec::new();
        for file in &self.args.files {
            files.push(PathBuf::from(file))
        }
        (self, files)
    }
}

