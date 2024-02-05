use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvoiceManagerError {
    #[error("Invoice store path not found: {0}")]
    InvoiceStorePathNotFound(String),

    #[error("Customer store path not found: {0}")]
    CustomerStorePathNotFound(String),

    #[error("Unable to initialize folder in: {0}")]
    UnableInitFolderInto(String),

    #[error("Unable to create directory: {0}")]
    UnableToCreateDirectory(String, #[source] Error),

    #[error("Unable to write customer file: {0}")]
    UnableToWriteCustomerFile(String, #[source] Error),
}
