use std::fs::{File};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Stdin, Stdout, StdoutLock, Write};
use std::path::PathBuf;
use std::process::exit;
use atty::Stream;
use clap::lazy_static::lazy_static;
use regex::Regex;
use crate::{CrabArgs};

pub struct Reader {
    args: CrabArgs,
    counter: usize,
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
            last_line: String::new(),
        }
    }

    pub fn read_file(&mut self, path: &PathBuf) {
        if !path.exists() {
            eprintln!("{} must exist", path.display());
            exit(1)
        }
        if !path.is_file() {
            eprintln!("{} must be a file", path.display());
            exit(2)
        }
        match File::open(path) {
            Ok(file) => {
                match self.read(file) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Error reading the file {}: {}", path.display(), err.to_string());
                        exit(err.raw_os_error().unwrap_or(5))
                    }
                }
            }
            Err(err) => {
                eprintln!("Unable to open file {}: {}", path.display(), err.to_string());
                exit(err.raw_os_error().unwrap_or(4))
            }
        }
    }

    pub fn read_stdin(&mut self) {
        if atty::isnt(Stream::Stdin) {
            let stdin: Stdin = io::stdin();
            match self.read(stdin.lock()) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error reading the {}: {}", "stdin", err.to_string());
                    exit(err.raw_os_error().unwrap_or(6))
                }
            }
        }
    }

    fn read<T: Read>(&mut self, inner: T) -> Result<(), std::io::Error> {
        lazy_static! {static ref TAB: Regex = Regex::new(r"[\t]").unwrap();}
        lazy_static! {static ref NEW_LINE: Regex = Regex::new(r"[\n]").unwrap(); }
        let mut reader: BufReader<T> = BufReader::new(inner);
        let stdout: Stdout = std::io::stdout();
        let locked: StdoutLock = stdout.lock();
        let mut buf: BufWriter<StdoutLock> = BufWriter::new(locked);
        loop {
            let mut line: Vec<u8> = Vec::new();
            let is_empty: bool;
            let result = reader.read_until(b'\n', &mut line);
            match result {
                Ok(0) => { return Ok(()); }
                Ok(n) => { is_empty = n <= 2 && (line == vec![b'\r', b'\n'] || line[0] == b'\n' || line[0] == b'\r') }
                Err(err) => {
                    eprintln!("Unable to read line: {}", err.to_string());
                    exit(err.raw_os_error().unwrap_or(3))
                }
            }
            let mut line: String = if self.args.show_non_printing {
                line.iter().map(
                    |c| match c {
                        0..=8 | 11..=31 => format!("^{}", (c + 64u8) as char),
                        127 => String::from("^?"),
                        128..=159 => format!("M-^{}", (c - 128 + 64u8) as char),
                        160..=254 => format!("M-{}", (c - 160 + 32u8) as char),
                        255 => String::from("M-^?"),
                        _ => String::from(*c as char)
                    }
                ).collect()
            } else {
                match String::from_utf8(line) {
                    Ok(line) => line,
                    Err(err) => {
                        eprintln!("Unable to read line: {}", err.to_string());
                        exit(6)
                    }
                }
            };
            if self.args.show_tabs { line = TAB.replace_all(&line, "^I").to_string(); }
            if self.args.show_ends { line = NEW_LINE.replace_all(&line, "$\n").to_string(); }

            if self.args.squeeze_blank && self.last_line == line && is_empty { continue; }
            self.last_line = line.to_string();
            if self.args.number_lines {
                let skip = self.args.number_non_blank && is_empty;
                if skip {
                    write!(buf, "{}", self.last_line)?;
                } else {
                    write!(buf, "{:>6}\t{}", self.counter, self.last_line)?;
                    self.counter += 1;
                }
            } else {
                write!(buf, "{}", self.last_line)?;
            }
        }
    }

    pub fn get_files(&mut self) -> Vec<PathBuf> {
        let mut files = Vec::with_capacity(self.args.files.capacity());
        for file in &self.args.files { files.push(PathBuf::from(file)) }
        files
    }
}