use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsFileManagerError {
    #[error("Unable to write settings file: {0}")]
    UnableToWriteFile(String, #[source] std::io::Error),
    #[error("Unable to read settings file: {0}")]
    UnableToReadPath(String, #[source] std::io::Error),
}
