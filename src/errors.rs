use crate::errors::CrabError::ReadError;
use crate::InputSource;
use std::env::current_exe;
use std::io::Error;
use std::path::PathBuf;
use CrabError::OpenError;

/// Errors that Crab can handle
pub enum CrabError {
    OpenError(String, Error),
    ReadError(InputSource, Error),
}

impl CrabError {
    pub fn show(&mut self) {
        let exe: PathBuf = current_exe().unwrap_or(PathBuf::from("crab"));
        eprint!("{}: ", exe.display());
        let message: String = match self {
            OpenError(path, err) => format!("{}: {}", path, err),
            ReadError(source, err) => {
                let name: String = match source {
                    InputSource::File(file) => format!("file {}", file),
                    InputSource::Stdin => String::from("Stdin"),
                };
                format!("Error reading {}: {}", name, err)
            }
        };
        eprintln!("{}", message);
    }
}
