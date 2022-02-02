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
        let file: File = File::open(file).expect(&format!("Unable to open file {}", file.display())[..]);
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
        let mut end = if self.args.show_ends { "$\n" } else { "\n" }.to_owned();
        let tab = if self.args.show_tabs { "^I" } else { "\t" };
        let mut lines = reader.lines();
        let mut line = lines.next();
        let mut new_line = true;
        loop {
            if matches!(line, Option::None) { return self; }
            let next = lines.next();
            if matches!(next, Option::None) {
                new_line = false;
                end.pop();
            }
            let line_str: String = line.unwrap().expect("Unable to read line").replace("\t", tab);
            let line_str = if self.args.show_non_printing { self.show_non_printable(line_str.as_bytes()) } else { line_str };
            if self.args.squeeze_blank && self.last_line == line_str && line_str.is_empty() {
                line = next;
                continue;
            }
            self.last_line = line_str;
            if self.args.number_lines {
                self.print_numbered(self.args.number_non_blank && self.last_line.is_empty(), &end[..]);
            } else { print!("{}{}", self.last_line, end) }
            if !new_line { break }
            line = next;
        }
        return self
    }

    fn print_numbered(&mut self, skip: bool, end: &str) {
        if skip { return print!("{}", end); }
        print!("{:>6}\t{}{}", self.counter, self.last_line, end);
        self.counter += 1;
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

