use crate::entities::customer::Customer;
use crate::file_manager::file_manager::{FileManager, Manager};
use dialoguer::Input;
use log::trace;
use std::error::Error;
use std::path::Path;

pub fn create_customer(
    invoice_manager_path: &Path,
    invoice_path: Option<&Path>,
    config_file_path: Option<&Path>,
    customer_file_path: Option<&Path>,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Create customer");

    let name: String =
        Input::new().with_prompt("Enterprise name").interact_text().unwrap();

    let address = Input::new().with_prompt("Address").interact_text().unwrap();

    let city = Input::new().with_prompt("City").interact_text().unwrap();

    let postal = Input::new().with_prompt("Postal code").interact_text().unwrap();

    let customer = Customer { name, address, postal, city };

    let file_manager = FileManager::new(
        invoice_manager_path,
        invoice_path,
        customer_file_path,
        config_file_path,
    )?;

    let customer = file_manager.create_customer(customer)?;

    println!("Customer {} created", customer.name);

    Ok(())
}
