use std::error::Error;

use log::trace;

use crate::cli::utils::select_customer_or_use_default::select_customer_or_use_default;
use crate::entities::customer::Customer;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;

pub fn get_customer(context_parameters: ContextParameters, customer_ref: &Option<String>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get customer");

    let file_manager = FileManager::new(context_parameters)?;

    let customer_selected: Customer = select_customer_or_use_default(&file_manager, customer_ref)?.1;

    println!("Your customer : {}\n", customer_selected.name);

    println!("Address : \n{}\n{} {}", customer_selected.address, customer_selected.postal, customer_selected.city);

    Ok(())
}
