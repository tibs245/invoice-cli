use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Customer {0} not found")]
    CustomerNotFound(String),

    #[error("Not implemented yet")]
    NotImplementedYet(),

    #[error("{0}")]
    CommandNotExists(String),
}