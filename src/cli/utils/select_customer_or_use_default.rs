use std::error::Error;

use crate::cli::cli_error::CliError;
use crate::cli::utils::select_customer::select_customer;
use crate::entities::customer::Customer;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub(crate) fn select_customer_or_use_default(file_manager: &FileManager, customer_ref: &Option<String>) -> Result<(String, Customer), Box<dyn Error + Sync + Send + 'static>> {
    if let Some(customer_preselected) = customer_ref {
        match file_manager.get_all_customers() {
            Ok(customer) => {
                match customer.get(customer_preselected) {
                    Some(customer) => Ok((customer_preselected.to_owned(), customer.to_owned())),
                    None => Err(Box::new(CliError::CustomerNotFound(customer_preselected.to_owned())))
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    } else {
        select_customer(&file_manager)
    }
}
