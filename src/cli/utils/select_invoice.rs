use std::error::Error;

use dialoguer::FuzzySelect;
use crate::cli::utils::cli_utils_error::CliUtilsError;

use crate::entities::invoice::Invoice;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub(crate) fn select_invoice(file_manager: &FileManager) -> Result<Invoice, Box<dyn Error + Sync + Send + 'static>> {
    let all_invoices: Vec<Invoice> = file_manager.get_all_invoices()?;

    if all_invoices.len() == 0 {
        return Err(Box::new(CliUtilsError::NoInvoiceFound()))
    }
     
    let invoice_index = FuzzySelect::new()
        .with_prompt("What is your invoice?")
        .items(&all_invoices)
        .interact()
        .unwrap();

    return Ok(all_invoices[invoice_index].clone());
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::cli::utils::cli_utils_error::CliUtilsError;

    use crate::entities::invoice::Invoice;
    use crate::file_manager::file_manager::InvoiceManager;

    pub(crate) fn mock_select_invoice(file_manager: &impl InvoiceManager) -> Result<Invoice, Box<dyn Error + Sync + Send + 'static>> {
        let all_invoices: Vec<Invoice> = file_manager.get_all_invoices()?;
        
        if all_invoices.len() == 0 {
            return Err(Box::new(CliUtilsError::NoInvoiceFound()))
        }
        
        Ok(all_invoices.get(0).unwrap().clone())
    }
}