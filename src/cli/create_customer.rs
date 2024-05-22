use std::error::Error;

use dialoguer::Input;
use log::trace;

use crate::entities::customer::Customer;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn create_customer(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Create customer");

    let name: String =
        Input::new().with_prompt("Enterprise name").interact_text().unwrap();

    let address = Input::new().with_prompt("Address").interact_text().unwrap();

    let city = Input::new().with_prompt("City").interact_text().unwrap();

    let postal = Input::new().with_prompt("Postal code").interact_text().unwrap();

    let customer = Customer { name, address, postal, city };

    let file_manager = FileManager::new(context_parameters)?;

    let customer = file_manager.create_customer(customer)?;

    println!("Customer {} created", customer.name);

    Ok(())
}
