use std::error::Error;

use dialoguer::Input;
use log::trace;

use crate::cli::utils::select_customer_or_use_default::select_customer_or_use_default;
use crate::entities::customer::Customer;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn edit_customer(context_parameters: ContextParameters, customer_ref: &Option<String>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Create customer");

    let file_manager = FileManager::new(context_parameters)?;

    let (customer_ref_selected, customer_to_edit) = select_customer_or_use_default(&file_manager, customer_ref)?;

    let name: String =
        Input::new().with_prompt("Enterprise name").with_initial_text(&customer_to_edit.name).interact_text().unwrap();

    let address = Input::new().with_prompt("Address").with_initial_text(&customer_to_edit.address).interact_text().unwrap();

    let city = Input::new().with_prompt("City").with_initial_text(&customer_to_edit.city).interact_text().unwrap();

    let postal = Input::new().with_prompt("Postal code").with_initial_text(&customer_to_edit.postal).interact_text().unwrap();

    let customer = Customer { name, address, postal, city };


    let customer = file_manager.edit_customer(customer_ref_selected, customer)?;

    println!("Customer {} edited", customer.name);

    Ok(())
}
