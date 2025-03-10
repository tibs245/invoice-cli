use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvoiceCustomerManagerError {
    #[error("Unable to find the customer file in: {0}")]
    CustomerStorePathNotFound(String),

    #[error("Unable to read customer file: {0}")]
    UnableToReadPath(String, #[source] std::io::Error),

    #[error("Unable to write customer file: {0}")]
    UnableToWriteCustomerFile(String, #[source] std::io::Error),

    #[error("Unable to create customer with duplicated id: {1} in {0}")]
    UnableCreateCustomerDuplicatedId(String, String),

    #[error("Unable to edit customer with id not found: {0}")]
    UnableEditCustomerDuplicatedId(String),

    #[error("Unable to delete customer with ref : `{0}`: Ref not found")]
    UnableDeleteCustomerRefNotFound(String),
}
