use crate::errors::CrabError::ReadError;
use crate::InputSource;
use std::env::current_exe;
use std::io::Error;
use std::path::PathBuf;
use CrabError::OpenError;

/// Errors that Crab can handle
pub enum CrabError {
    /// Error opening a file. It contains the path and the underlying error.
    OpenError(String, Error),
    /// Error reading from a source. It contains the source and the underlying error.
    ReadError(InputSource, Error),
}

impl CrabError {
    /// Displays the error message to standard error output in the format:
    /// "[executable name]: [error message]"
    pub fn show(&mut self) {
        // Get the name of the executable, or use "crab" if that fails
        let exe: PathBuf = current_exe().unwrap_or(PathBuf::from("crab"));

        // Print the executable name and a colon on stderr
        eprint!("{}: ", exe.display());

        // Generate the error message based on the type of error
        let message: String = match self {
            // If the error is an OpenError, format the message as "[path]: [error]"
            OpenError(path, err) => format!("{}: {}", path, err),
            // If the error is a ReadError, format the message based on the input source
            ReadError(source, err) => {
                let name: String = match source {
                    // If the source is a file, format the name as "file [path]"
                    InputSource::File(file) => format!("file {}", file),

                    // If the source is stdin, use the name "Stdin"
                    InputSource::Stdin => String::from("Stdin"),
                };
                // Combine the name and the error message with a colon
                format!("Error reading {}: {}", name, err)
            }
        };
        // Print the error message to standard error output
        eprintln!("{}", message);
    }
}
