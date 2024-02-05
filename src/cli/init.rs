use crate::file_manager::file_manager::FileManager;
use log::trace;
use std::error::Error;
use std::path::Path;

pub fn initiate_invoice_directory(
    invoice_manager_path: &Path,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Initiate invoice directory");
    FileManager::init(invoice_manager_path, None, None, None)?;

    Ok(())
}
