use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliUtilsError {
    #[error("No invoice already created found")]
    NoInvoiceFound(),
}