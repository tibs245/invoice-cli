use std::error::Error;

use log::trace;

use crate::cli::utils::select_invoice_or_use_default::select_invoice_or_use_default;
use crate::entities::invoice::Invoice;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn generate_invoice(context_parameters: ContextParameters, invoice_ref: &Option<String>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get invoice");

    let file_manager = FileManager::new(context_parameters.clone())?;

    let invoice_selected: Invoice = select_invoice_or_use_default(&file_manager, invoice_ref)?;

    let mut invoice_input_path = file_manager.get_invoice_path().to_owned().join(invoice_selected.get_ref().unwrap());

    invoice_input_path.set_extension("yaml");

    let invoice_output_name = invoice_selected.get_ref().unwrap() + ".pdf";

    let output_path = file_manager.generate_invoice(&(invoice_input_path.as_path()), &invoice_output_name)?;

    println!("Invoice generated in : {}", output_path.to_string_lossy());
    return Ok(());
}
