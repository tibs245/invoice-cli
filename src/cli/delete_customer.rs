use std::error::Error;

use chrono::Local;
use log::trace;

use crate::cli::context_parameters::ContextParameters;
use crate::cli::utils::select_customer_or_use_default::select_customer_or_use_default;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub fn delete_customer(context_parameters: ContextParameters, customer_ref: &Option<String>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Delete customer");

    let file_manager = FileManager::new(
        context_parameters.invoice_manager_path,
        context_parameters.invoice_path,
        context_parameters.customer_file_path,
        context_parameters.config_file_path,
    )?;
    
    let customer_ref_selected = select_customer_or_use_default(&file_manager, customer_ref)?.0;

    let result = file_manager.remove_customer(&customer_ref_selected);
    
    if result.is_ok() {
        println!("Customer {} deleted", customer_ref_selected);
    }
    
    result
}
