use std::error::Error;

use log::trace;

use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn list_customers(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get customers");

    let file_manager = FileManager::new(context_parameters)?;

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
