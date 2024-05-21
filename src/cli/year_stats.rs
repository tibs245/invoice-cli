use std::error::Error;

use chrono::Datelike;
use log::trace;

use crate::cli::context_parameters::ContextParameters;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub fn year_stats(context_parameters: ContextParameters, year: &Option<i32>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get year stats");

    let year = year.unwrap_or(chrono::Local::now().year());

    let file_manager = FileManager::new(
        context_parameters.invoice_manager_path,
        context_parameters.invoice_path,
        context_parameters.customer_file_path,
        context_parameters.config_file_path,
    )?;

    let all_year_invoices = file_manager.get_invoice_by_year(year)?;

    let total_cost = all_year_invoices.iter()
        .fold(0.0, |total, invoice| total + invoice.get_total_price());

    let mut plural_offset = "";
    if all_year_invoices.len() > 1 {
        plural_offset = "s";
    }

    println!("Get {} invoice{} for {}\n", all_year_invoices.len(), plural_offset, year);

    all_year_invoices
        .iter()
        .for_each(|invoice| println!("{}", invoice));

    println!("Total : {} €", total_cost);

    Ok(())
}
