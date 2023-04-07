use crate::errors::CrabError::{OpenError, ReadError};
use atty::Stream;
use std::fs::{File};
use std::io::{BufReader, Read, Result, stdin};
use std::path::PathBuf;

#[derive(Clone)]
/// Represents a source of input.
pub enum InputSource {
    /// Standard input.
    Stdin,
    /// A file.
    File(String),
}

/// Represents a reader that can read input from multiple sources.
pub struct Reader {
    /// The list of input sources to read from.
    sources: Vec<InputSource>,
    /// The list of file readers, corresponding to the `File` input sources.
    readers: Vec<BufReader<File>>,
    /// The exit code to return if an error occurs while reading.
    pub exit_code: i32,
}

impl Read for Reader {
    /// Attempts to read data from the input sources into the given buffer.
    ///
    /// This method reads from the first input source in the list that contains data,
    /// and removes that source from the list once it has been exhausted.
    /// If an error occurs while reading, the source is removed from the list
    /// and an error message is printed to the standard error stream.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the input sources cannot be read from,
    /// or if an I/O error occurs while reading.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut r = 0;

        // Loop until data is read or there are no more input sources.
        while r == 0 && !self.sources.is_empty() {
            match &self.sources[0] {
                InputSource::File(_) => {
                    // If the current input source is a file, read from it.
                    match self.readers[0].read(buf) {
                        Ok(n) => {
                            r = n;
                        }
                        Err(err) => {
                            // If an error occurs, remove the source and print an error message.
                            ReadError(self.sources[0].clone(), err).show();
                            self.sources.remove(0);
                            self.readers.remove(0);
                            continue;
                        }
                    }

                    if r == 0 {
                        // If no data was read, remove the source.
                        self.sources.remove(0);
                        self.readers.remove(0);
                    }
                }
                InputSource::Stdin => {
                    // If the current input source is stdin, check if it is a TTY.
                    if atty::isnt(Stream::Stdin) {
                        // If it is not a TTY, read from it.
                        match stdin().lock().read(buf) {
                            Ok(n) => {r=n}
                            Err(err) => {
                                // If an error occurs, remove the source and print an error message.
                                ReadError(self.sources[0].clone(), err).show();
                                self.sources.remove(0);
                                continue;
                            }
                        }
                        if r == 0 {
                            // If no data was read, remove the source.
                            self.sources.remove(0);
                        }
                    } else {
                        // If it is a TTY, remove the source.
                        self.sources.remove(0);
                    }
                }
            }
        }

        // Return the number of bytes read.
        Ok(r)
    }
}

impl Reader {
    /// Create a new `Reader` instance.
    ///
    /// This method takes a vector of `InputSource` enums as input, which represent
    /// the sources from which data will be read. The method creates a new `Reader`
    ///
    /// # Arguments
    ///
    /// * `s` - A vector of `InputSource` enums representing the sources to read from.
    ///
    /// # Returns
    ///
    /// A new `Reader` instance.
    pub fn new(s: Vec<InputSource>) -> Self {
        let mut exit_code = 0;
        let mut sources = vec![];
        // Create a new Vec of BufReaders by iterating over each InputSource in the input vector
        // and opening a corresponding file, if applicable.
        let readers = s.iter()
            .map(|i| match i {
                InputSource::File(file) => {
                    // Open the file and return a BufReader for valid files
                    if let Some(_file) = Self::open_file(file.to_string()) {
                        // Push the valid source to the sources vec
                        (&mut sources).push(i.clone());
                        Some(BufReader::new(_file))
                    } else {
                        // Set the exit code to 1 if the file could not be opened
                        exit_code = 1;
                        None
                    }
                }
                InputSource::Stdin => {
                    // Push the valid source to the sources vec
                    (&mut sources).push(i.clone());
                    None
                }
            })
            .filter(|o| matches!(o, Some(_)))
            .map(|o| o.unwrap())
            .collect::<Vec<_>>();
        Reader {
            sources,
            readers,
            exit_code,
        }
    }

    /// Open a file and return a File handle.
    ///
    /// This method takes a string representing a file path, and returns a `File` handle
    /// for the file at that path, or `None` if the file could not be opened. If an error
    /// occurs while opening the file, the method prints an error message using the `OpenError`
    /// struct and returns `None`.
    ///
    /// # Arguments
    ///
    /// * `_path` - A string representing the path of the file to open.
    ///
    /// # Returns
    ///
    /// A `File` handle for the opened file, or `None` if the file could not be opened.
    fn open_file(_path: String) -> Option<File> {
        let path: PathBuf = PathBuf::from(&_path);
        match File::open(path) {
            Ok(file) => Some(file),
            Err(err) => {
                // Print an error message if the file could not be opened
                OpenError(_path, err).show();
                None
            }
        }
    }
}
