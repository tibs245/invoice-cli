use std::error::Error;

use chrono::Datelike;
use log::trace;

use crate::entities::invoice_date::MonthString;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn month_stats(context_parameters: ContextParameters, month: &Option<u32>, year: &Option<i32>) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Get month stats");

    let year = year.unwrap_or(chrono::Local::now().year());
    let month = month.unwrap_or(chrono::Local::now().month());

    let file_manager = FileManager::new(context_parameters)?;

    let all_month_invoices = file_manager.get_invoice_by_month(year, month)?;

    let total_cost = all_month_invoices.iter()
        .fold(0.0, |total, invoice| total + invoice.get_total_price());

    let mut plural_offset = "";
    if all_month_invoices.len() > 1 {
        plural_offset = "s";
    }

    println!("Get {} invoice{} for {}/{}\n", all_month_invoices.len(), plural_offset, MonthString::new(&month.to_string()).unwrap().to_string(), year);

    all_month_invoices
        .iter()
        .for_each(|invoice| println!("{}", invoice));

    println!("Total : {} €", total_cost);

    Ok(())
}
