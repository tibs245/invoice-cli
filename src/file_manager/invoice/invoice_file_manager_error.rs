use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvoiceFileManagerError {
    #[error("Invoice store path not found: {0}")]
    InvoiceStorePathNotFound(String),

    #[error("Unable to write invoice file")]
    UnableToWriteInvoiceFile(#[source] Error),

    #[error("Unable to read path: {0}")]
    UnableToReadPath(String, #[source] Error),
}
