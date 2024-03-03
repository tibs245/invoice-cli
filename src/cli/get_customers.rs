use crate::file_manager::file_manager::{FileManager, Manager};
use log::trace;
use std::error::Error;
use std::path::Path;

pub fn get_customers(
    invoice_manager_path: &Path,
    invoice_path: Option<&Path>,
    config_file_path: Option<&Path>,
    customer_file_path: Option<&Path>,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get customers");

    let file_manager = FileManager::new(
        invoice_manager_path,
        invoice_path,
        customer_file_path,
        config_file_path,
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
