use std::error::Error;

use log::trace;

use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliError::InvoiceRefNotFound;
use crate::cli::utils::select_invoice::select_invoice;
use crate::entities::invoice::Invoice;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn get_invoice(context_parameters: ContextParameters, invoice_ref: &Option<String>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get invoice");

    let file_manager = FileManager::new(context_parameters.clone())?;

    let invoice_selected: Invoice = if let Some(invoice_preselected) = invoice_ref {
        match file_manager.get_invoice_by_ref(&invoice_preselected) {
            Ok(invoice) => Ok(invoice),
            Err(_) => {
                Err(Box::new(InvoiceRefNotFound(invoice_preselected.to_string())))
            }
        }
    } else {
        match select_invoice(&file_manager) {
            Ok(invoice) => Ok(invoice),
            Err(_) => {
                Err(Box::new(CliError::UnableLoadInvoiceFolder(context_parameters.invoice_path.unwrap().to_string_lossy().to_string())))
            }
        }
    }?;


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
