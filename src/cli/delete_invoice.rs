use std::error::Error;

use chrono::Local;
use log::trace;

use crate::cli::utils::select_invoice_or_use_default::select_invoice_or_use_default;
use crate::entities;
use crate::entities::product::Product;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn cancel_invoice(context_parameters: ContextParameters, invoice_ref: &Option<String>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Cancel invoice");

    let file_manager = FileManager::new(context_parameters)?;

    let invoice_selected = select_invoice_or_use_default(&file_manager, invoice_ref)?;

    let cancel_invoice = entities::invoice::Invoice {
        date: Local::now().date_naive(),
        customer_id: invoice_selected.customer_id.to_owned(),
        title: format!("Cancel : {} ({})", invoice_selected.title, invoice_selected.get_ref().unwrap()),
        invoice_day_id: None,
        products: invoice_selected.products.iter().map(|product| Product { description: product.description.to_owned(), quantity: product.quantity, price: product.price * -1.0 }).collect(),
    };


    let invoice = file_manager.create_invoice(cancel_invoice)?;

    println!("Cancel Invoice created at : {}", invoice.to_string_lossy());

    Ok(())
}
