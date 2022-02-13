use crate::errors::CrabError::ReadError;
use crate::InputSource;
use std::env::current_exe;
use std::io::Error;
use std::path::PathBuf;
use std::process::exit;
use CrabError::OpenError;

/// Errors that Crab can handle
pub enum CrabError {
    OpenError(String, Error),
    ReadError(InputSource, Error),
}

impl CrabError {
    pub fn show_and_exit(&mut self) -> ! {
        let exe = current_exe().unwrap_or(PathBuf::from("crab"));
        eprint!("{}: ", exe.display());
        let (message, exit_code) = match self {
            OpenError(path, err) => (format!("{}: {}", path, err), 1),
            ReadError(source, err) => {
                let name: String = match source {
                    InputSource::File(file) => format!("file {}", file),
                    InputSource::Stdin => String::from("Stdin"),
                };
                (format!("Error reading {}: {}", name, err), 2)
            }
        };
        eprintln!("{}", message);
        exit(exit_code)
    }
}
