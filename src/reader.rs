use crate::args::Args;
use crate::errors::CrabError;
use crate::errors::CrabError::ReadError;
use atty::Stream;
use io::stdin;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Error, Read, Stdout, StdoutLock, Write};
use std::path::PathBuf;
use CrabError::OpenError;

#[derive(Clone)]
pub enum InputSource {
    File(String),
    Stdin,
}

pub struct Reader {
    args: Args,
    counter: usize,
    last_line_empty: bool,
}

impl Reader {
    pub fn new(args: Args) -> Self {
        Reader {
            args,
            counter: 1,
            last_line_empty: false,
        }
    }

    fn open_file(_path: String) -> Option<File> {
        let path: PathBuf = PathBuf::from(&_path);
        match File::open(path) {
            Ok(file) => Some(file),
            Err(err) => {
                OpenError(_path, err).show();
                None
            }
        }
    }

    pub fn read_source(&mut self, source: InputSource) -> u8 {
        match &source {
            InputSource::File(file) => {
                if let Some(_file) = Self::open_file(file.to_string()) {
                    self.read(_file, source);
                } else {
                    return 1;
                }
            }
            InputSource::Stdin => {
                if atty::isnt(Stream::Stdin) {
                    self.read(stdin().lock(), source)
                }
            }
        }
        0
    }

    fn read<T: Read>(&mut self, inner: T, source: InputSource) {
        let mut reader: BufReader<T> = BufReader::new(inner);
        let stdout: Stdout = std::io::stdout();
        let locked: StdoutLock = stdout.lock();
        let mut buf: BufWriter<StdoutLock> = BufWriter::new(locked);
        loop {
            let mut line: Vec<u8> = Vec::new();
            let result: std::io::Result<usize> = reader.read_until(b'\n', &mut line);
            match result {
                Ok(0) => {
                    return;
                }
                Ok(_n) => {}
                Err(err) => {
                    ReadError(source, err).show();
                    return;
                }
            }
            if self.on_line(line, &mut buf).is_err() {
                return;
            }
        }
    }

    fn on_line(&mut self, data: Vec<u8>, buf: &mut BufWriter<StdoutLock>) -> Result<(), Error> {
        let tab = if self.args.show_tabs {
            &b"^I"[..]
        } else {
            &b"\t"[..]
        };
        let new_line = if self.args.show_ends {
            &[b'$', b'\n'][..]
        } else {
            &[b'\n'][..]
        };
        
        let is_empty = match data.as_slice() {
            b"\r\n" | b"\n" | b"\r" => true,
            _ => false,
        };

        if self.args.squeeze_blank && self.last_line_empty && is_empty {
            return Ok(());
        }
        self.last_line_empty = is_empty;

        if self.args.number_lines && !(self.args.number_non_blank && is_empty) {
            write!(buf, "{:>6}\t", self.counter)?;
            self.counter += 1;
        }
        if self.args.show_non_printing {
            for c in data {
                match c {
                    0..=8 | 11..=31 => buf.write_all(&[b'^', (c + 64)])?,
                    127 => buf.write_all(b"^?")?,
                    128..=159 => buf.write_all(&[b'M', b'-', b'^', c - 128 + 64])?,
                    160..=254 => buf.write_all(&[b'M', b'-', c - 160 + 32])?,
                    255 => buf.write_all(b"M-^?")?,
                    b'\t' => buf.write_all(tab)?,
                    b'\n' => buf.write_all(new_line)?,
                    _ => buf.write_all(&[c])?,
                }
            }
        } else {
            for c in data {
                match c {
                    b'\t' => buf.write_all(tab)?,
                    b'\n' => buf.write_all(new_line)?,
                    _ => buf.write_all(&[c])?,
                }
            }
        }
        Ok(())
    }
}
