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

    fn open_file(_path: String) -> File {
        let path: PathBuf = PathBuf::from(&_path);
        match File::open(path) {
            Ok(file) => file,
            Err(err) => OpenError(_path, err).show_and_exit(),
        }
    }

    pub fn read_source(&mut self, source: InputSource) {
        match &source {
            InputSource::File(file) => self.read(Self::open_file(file.to_string()), source),
            InputSource::Stdin => {
                if atty::isnt(Stream::Stdin) {
                    self.read(stdin().lock(), source)
                }
            }
        }
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
                Err(err) => ReadError(source, err).show_and_exit(),
            }
            if self.on_line(line, &mut buf).is_err() {
                return;
            }
        }
    }

    fn on_line(&mut self, data: Vec<u8>, buf: &mut BufWriter<StdoutLock>) -> Result<(), Error> {
        let tab = if self.args.show_tabs {
            &[b'^', b'I'][..]
        } else {
            &[b'\t'][..]
        };
        let new_line = if self.args.show_ends {
            &[b'$', b'\n'][..]
        } else {
            &[b'\n'][..]
        };
        let is_empty = data.len() <= 2
            && (data[..] == [b'\r', b'\n'][..] || data[0] == b'\n' || data[0] == b'\r');

        if self.args.squeeze_blank && self.last_line_empty && is_empty {
            return Ok(());
        }
        self.last_line_empty = is_empty;

        let mut buffer: Vec<u8> = Vec::with_capacity(if self.args.show_non_printing {
            1024 * 1024
        } else {
            data.len()
        });
        if self.args.number_lines && !(self.args.number_non_blank && is_empty) {
            write!(buffer, "{:>6}\t", self.counter)?;
            self.counter += 1;
        }
        if self.args.show_non_printing {
            for c in data {
                match c {
                    0..=8 | 11..=31 => buffer.write_all(&[b'^', (c + 64)])?,
                    127 => buffer.write_all(b"^?")?,
                    128..=159 => buffer.write_all(&[b'M', b'-', b'^', c - 128 + 64])?,
                    160..=254 => buffer.write_all(&[b'M', b'-', c - 160 + 32])?,
                    255 => buffer.write_all(b"M-^?")?,
                    b'\t' => buffer.write_all(tab)?,
                    b'\n' => buffer.write_all(new_line)?,
                    _ => buffer.write_all(&[c])?,
                }
            }
        } else {
            for c in data {
                match c {
                    b'\t' => buffer.write_all(tab)?,
                    b'\n' => buffer.write_all(new_line)?,
                    _ => buffer.write_all(&[c])?,
                }
            }
        }
        buf.write_all(buffer.as_slice())?;
        Ok(())
    }
}
