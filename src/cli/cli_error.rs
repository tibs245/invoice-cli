use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invoice ref {0} not found")]
    InvoiceRefNotFound(String),

    #[error("Customer {0} not found")]
    CustomerNotFound(String),

    #[error("Invoice to load invoice folder : {0}")]
    UnableLoadInvoiceFolder(String),

    #[error("Invoice to read customer file : {0}")]
    UnableReadCustomerFile(String),

    #[error("Not implemented yet")]
    NotImplementedYet(),

    #[error("{0}")]
    CommandNotExists(String),
}