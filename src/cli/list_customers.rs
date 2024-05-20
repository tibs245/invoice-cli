use std::error::Error;

use log::trace;

use crate::cli::context_parameters::ContextParameters;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub fn list_customers(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get customers");

    let file_manager = FileManager::new(
        context_parameters.invoice_manager_path,
        context_parameters.invoice_path,
        context_parameters.customer_file_path,
        context_parameters.config_file_path,
    )?;

    let all_customers = file_manager.get_all_customers()?;

    let mut plural_offset = "";
    if all_customers.len() > 1 {
        plural_offset = "s";
    }

    println!("Get {} customer{}\n", all_customers.len(), plural_offset);

    all_customers
        .iter()
        .for_each(|(_, customer)| println!("{}", customer.name));

    Ok(())
}
