use std::io::{BufWriter, Error, Stdout, StdoutLock};
use std::io::Write;
use crossbeam::{Receiver};
use crate::CrabArgs;

/// Structure in charge of processing the text and writing it to the standard output
pub struct Writer {
    args: CrabArgs,
    counter: usize,
    last_line_empty: bool,
    receiver: Receiver<Vec<u8>>,
}

impl Writer {
    pub fn new(args: CrabArgs, receiver: Receiver<Vec<u8>>) -> Self {
        Writer {
            args,
            counter: 1,
            last_line_empty: false,
            receiver,
        }
    }

    pub fn write(&mut self) {
        let stdout: Stdout = std::io::stdout();
        let locked: StdoutLock = stdout.lock();
        let mut buf: BufWriter<StdoutLock> = BufWriter::new(locked);
        loop {
            match self.receiver.recv() {
                Ok(data) => {
                    match self.on_line(data, &mut buf) {
                        Ok(_) => {}
                        Err(_) => {return;}
                    }
                }
                Err(_) => { return; }
            }
        }
    }

    fn on_line(&mut self, data: Vec<u8>, buf: &mut BufWriter<StdoutLock>) -> Result<(), Error> {
        let tab = &if self.args.show_tabs { vec![b'^', b'I'] } else { vec![b'\t'] }[..];
        let new_line = &if self.args.show_ends { vec![b'$', b'\n'] } else { vec![b'\n'] }[..];
        let is_empty = data.len() <= 2 && (data == vec![b'\r', b'\n'] || data[0] == b'\n' || data[0] == b'\r');

        if self.args.squeeze_blank && self.last_line_empty && is_empty { return Ok(()); }
        self.last_line_empty = is_empty;

        if self.args.number_lines && !(self.args.number_non_blank && is_empty) {
            write!(buf, "{:>6}\t", self.counter)?;
            self.counter += 1;
        }

        if self.args.show_non_printing {
            for c in data {
                match c {
                    0..=8 | 11..=31 => { buf.write_all(&[b'^', (c + 64)][..])? }
                    127 => { buf.write_all(&b"^?"[..])? }//String::from("^?"),
                    128..=159 => { buf.write_all(&[b'M', b'-', b'^', c - 128 + 64][..])? }
                    160..=254 => { buf.write_all(&[b'M', b'-', c - 160 + 32][..])? }
                    255 => { buf.write_all(&b"M-^?"[..])? }
                    b'\t' => { buf.write_all(tab)? }
                    b'\n' => { buf.write_all(new_line)? }
                    _ => { buf.write_all(&[c][..])? }
                }
            }
        } else {
            for c in data {
                match c {
                    b'\t' => { buf.write_all(tab)? }
                    b'\n' => { buf.write_all(new_line)? }
                    _ => { buf.write_all(&[c][..])? }
                }
            }
        }
        Ok(())
    }
}