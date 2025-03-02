use std::error::Error;

use chrono::Local;
use dialoguer::{Confirm, FuzzySelect, Input};
use log::trace;

use crate::entities::invoice::Invoice;
use crate::entities::product::Product;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;
use crate::mistral::provider::extract_invoice_params;

pub async fn mistral_create_invoice(context_parameters: ContextParameters<'_>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Create invoice from LLM");

    let file_manager = FileManager::new(context_parameters)?;

    let prompt: String = Input::new().with_prompt("What is the invoice you want create ?").interact_text().unwrap();

    let invoice = extract_invoice_params(&file_manager, &prompt).await?;

    let invoice_path = file_manager.create_invoice(invoice)?;

    println!("Invoice created in : {}", invoice_path.to_string_lossy());

    let mut invoice_path_output = invoice_path.clone();
    invoice_path_output.set_extension("pdf");

    file_manager.generate_invoice(invoice_path.as_path(), invoice_path_output.file_name().unwrap().to_str().unwrap())?;

    Ok(())
}
