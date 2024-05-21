use std::error::Error;

use crate::cli::utils::select_invoice::select_invoice;
use crate::entities::invoice::Invoice;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub(crate) fn select_invoice_or_use_default(file_manager: &FileManager, invoice_ref: &Option<String>) -> Result<Invoice, Box<dyn Error + Sync + Send + 'static>> {
    if let Some(invoice_preselected) = invoice_ref {
         file_manager.get_invoice_by_ref(invoice_preselected)
    } else {
        select_invoice(&file_manager)
    }
}
