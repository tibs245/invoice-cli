use std::error::Error;

use log::trace;

use crate::cli::utils::select_invoice_or_use_default::select_invoice_or_use_default;
use crate::entities::invoice::Invoice;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;

pub fn get_invoice(context_parameters: ContextParameters, invoice_ref: &Option<String>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get invoice");

    let file_manager = FileManager::new(context_parameters.clone())?;

    let invoice_selected: Invoice = select_invoice_or_use_default(&file_manager, invoice_ref)?;

    println!("Your invoice : {}", invoice_selected.get_ref().unwrap().to_string());
    println!("{}\n", invoice_selected.title);
    println!("Date : {}", invoice_selected.date.to_string());

    println!("Products : ");
    for product in invoice_selected.products.iter() {
        println!(" - {} : {} * {}€ = {}€", product.description, product.quantity, product.price, product.get_total_price());
    }

    println!("\nTotal price : {} €", invoice_selected.get_total_price());

    Ok(())
}
