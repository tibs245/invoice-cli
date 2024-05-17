use std::error::Error;
use dialoguer::FuzzySelect;
use crate::entities::invoice::Invoice;
use crate::file_manager::file_manager::{FileManager, Manager};

pub(crate) fn select_invoice(file_manager: &FileManager) -> Result<Invoice, Box<dyn Error + Sync + Send + 'static>> {
    let all_invoices: Vec<Invoice> = file_manager.get_all_invoices()?;

    let invoice_index = FuzzySelect::new()
        .with_prompt("What is your customer?")
        .items(&all_invoices)
        .interact()
        .unwrap();

    return Ok(all_invoices[invoice_index].clone());
}