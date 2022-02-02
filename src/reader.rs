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

    pub fn read_file(&mut self, file: &PathBuf)  {
        if !file.exists() {
            eprintln!("{} must exist", file.display());
            exit(1)
        }
        if !file.is_file() {
            eprintln!("{} must be a file", file.display());
            exit(2)
        }
        match File::open(file)  {
            Ok(file) => {self.read(file)}
            Err(err) => {
                eprintln!("Unable to open file {}: {}", file.display(), err.to_string());
                exit(err.raw_os_error().unwrap_or(4))
            }
        }
    }

    pub fn read_stdin(&mut self)  {
        if atty::isnt(Stream::Stdin) {
            let stdin: Stdin = io::stdin();
            self.read(stdin.lock());
        }
    }

    fn read<T: Read>(&mut self, inner: T)  {
        let mut reader = BufReader::new(inner);
        let end = if self.args.show_ends { "$" } else { "" };
        let tab = if self.args.show_tabs { "^I" } else { "\t" };
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => { return }
                Ok(_n) => {}
                Err(err) => {
                    eprintln!("Unable to read line: {}", err.to_string());
                    exit(err.raw_os_error().unwrap_or(3))
                }
            }
            let line: String = line.replace("\t", tab);
            let line: String = if self.args.show_non_printing { self.show_non_printable(line.as_bytes()) } else { line };
            if self.args.squeeze_blank && self.last_line == line && line.is_empty() {continue;}
            self.last_line = line.replace("\n", format!("{}\n", end).as_str());
            if self.args.number_lines {
                self.print_numbered(self.args.number_non_blank && self.last_line.is_empty());
            } else { print!("{}", self.last_line) }
        }
    }

    fn print_numbered(&mut self, skip: bool) {
        if skip { return print!("{}",self.last_line); }
        print!("{:>6}\t{}", self.counter, self.last_line);
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

    pub fn get_files(&mut self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        for file in &self.args.files {
            files.push(PathBuf::from(file))
        }
        files
    }
}

