use std::error::Error;

use log::trace;

use crate::entities::settings::Settings;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn get_settings(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get settings");

    let file_manager = FileManager::new(context_parameters)?;

    let settings: Settings = file_manager.get_settings()?;

    println!("Your settings :\n");

    println!("Enterprise :");
    println!("Name: {}", settings.enterprise.name);
    println!("Siren Number: {}", settings.enterprise.siren); // Assuming Siren has a `number` field
    println!("Email: {}", settings.enterprise.email);
    println!("Address: {}", settings.enterprise.address);
    println!("City: {}", settings.enterprise.city);
    println!("Postal Code: {}", settings.enterprise.postal);
    println!("Phone: {}", settings.enterprise.phone);
    println!("Title: {}", settings.enterprise.title);

    println!("\nInvoice clauses :");

    println!("Politeness: {}", settings.politeness);
    println!("Law rules: {}", settings.law_rules);

    Ok(())
}
