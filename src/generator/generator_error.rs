use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Unable create output directory {0}")]
    UnableToCreateOutputDirectory(String, #[source] Error),
}