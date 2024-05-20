use std::error::Error;

use log::trace;

use crate::cli::context_parameters::ContextParameters;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub fn list_invoices(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== List invoices");

    let file_manager = FileManager::new(
        context_parameters.invoice_manager_path,
        context_parameters.invoice_path,
        context_parameters.customer_file_path,
        context_parameters.config_file_path,
    )?;

    let all_invoices = file_manager.get_all_invoices()?;

    let mut plural_offset = "";
    if all_invoices.len() > 1 {
        plural_offset = "s";
    }

    println!("Get {} invoice{}\n", all_invoices.len(), plural_offset);

    all_invoices
        .iter()
        .for_each(|invoice| println!("{}", invoice));

    Ok(())
}
