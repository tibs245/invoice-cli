use std::error::Error;

use log::trace;

use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn generate_all_invoice(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get invoice");

    let file_manager = FileManager::new(context_parameters.clone())?;

    let all_invoice = file_manager.get_all_invoices()?;


    all_invoice.iter().for_each(|invoice| {
        let mut invoice_input_path = file_manager.get_invoice_path().to_owned().join(invoice.get_ref().unwrap());

        invoice_input_path.set_extension("yaml");

        let invoice_output_name = invoice.get_ref().unwrap() + ".pdf";

        let output_path = file_manager.generate_invoice(&(invoice_input_path.as_path()), &invoice_output_name);

        println!("Invoice generated in : {}", output_path.unwrap().to_string_lossy());
    });

    return Ok(());
}
