use std::io::{BufWriter, stdout, StdoutLock, Write};
use crate::args::ARGS;
use crate::mapping::{new_line, tab};

/// The OutWriter struct represents a buffered writer for standard output with additional features
/// such as line numbering and showing non-printing characters.
pub struct OutWriter {
    /// The underlying buffer writer for standard output.
    writer: BufWriter<StdoutLock<'static>>,
    /// An array of size 2 that stores the last two characters written to the output stream.
    last_two_chars: [u8; 2],
    /// A counter for the number of lines written to the output stream.
    lines_count: usize,
    /// A flag that indicates whether the last line written to the output stream was empty.
    last_line_empty: bool
}

impl OutWriter {
    /// Creates a new instance of `OutWriter`.
    pub fn new() -> Self {
        Self {
            writer: BufWriter::new(stdout().lock()),
            last_two_chars: [0,0],
            lines_count: 1,
            last_line_empty: false,
        }
    }
}

impl Write for OutWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.write_all(buf) {
            Ok(_) => Ok(buf.len()),
            Err(e) => Err(e)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> { self.writer.flush() }

    /// Write all bytes in the given buffer to the underlying writer. Special characters are handled based
    /// on the flags provided in the `ARGS` global variable. If the `squeeze_blank` flag is set, empty lines
    /// are skipped if the last line and the current line are both empty. If the `number_lines` flag is set,
    /// lines are numbered, and if the `number_non_blank` flag is set, only non-empty lines are numbered.
    ///
    /// # Arguments
    ///
    /// * `buf` - A slice of bytes to write to the underlying writer.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if there is a problem writing to the underlying writer.
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        for character in buf {
            // Check if the current line is empty
            let is_empty = matches!(
                (character, self.last_two_chars),
                (b'\n', [b'\n', b'\r']) | (b'\n', [_, b'\n']) | (b'\n', [0, 0])
            );
            // If the `squeeze_blank` flag is set and the last line was empty and the current line is empty, skip this character
            if ARGS.squeeze_blank && self.last_line_empty && is_empty {
                continue
            }
            self.last_line_empty = is_empty;

            // If the `number_lines` flag is set and the current line should be numbered, output the line number
            if ARGS.number_lines && !(ARGS.number_non_blank && is_empty) && (self.last_two_chars[1] == b'\n' || matches!(self.last_two_chars, [0,0])) {
                write!(self.writer, "{:>6}\t", self.lines_count)?;
                self.lines_count += 1;
            }

            // Output the current character according to the settings and any special character handling
            match character {
                b'\t' => self.writer.write_all(tab()),
                b'\n' => self.writer.write_all(new_line()),
                _ if !ARGS.show_non_printing => self.writer.write_all(&[*character]),
                0..=8 | 11..=31 => self.writer.write_all(&[b'^', character + 64]),
                127 => self.writer.write_all(b"^?"),
                128..=159 => self.writer.write_all(&[b'M', b'-', b'^', character - 64]),
                160..=254 => self.writer.write_all(&[b'M', b'-', character - 128]),
                255 => self.writer.write_all(b"M-^?"),
                _ => self.writer.write_all(&[*character])
            }?;

            // Update the last two characters written with the current character
            self.last_two_chars[0] = self.last_two_chars[1];
            self.last_two_chars[1] = *character;
        }
        Ok(())
    }
}