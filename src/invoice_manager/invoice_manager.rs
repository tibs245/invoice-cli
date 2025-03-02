use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;

use crate::entities::customer::Customer;
use crate::entities::invoice::Invoice;
use crate::entities::settings::Settings;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait InvoiceManager {
    fn create_invoice(
        &self,
        invoice: Invoice,
    ) -> Result<PathBuf, Box<dyn Error + Sync + Send + 'static>>;
    fn get_all_invoices(
        &self,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>>;
    fn get_invoice_by_ref(
        &self,
        invoice_reference: &str,
    ) -> Result<Invoice, Box<dyn Error + Sync + Send + 'static>>;
    fn get_invoice_by_date(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>>;
    fn get_invoice_by_month(
        &self,
        year: i32,
        month: u32,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>>;
    fn get_invoice_by_year(
        &self,
        year: i32,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>>;
    fn get_all_customers(
        &self,
    ) -> Result<HashMap<String, Customer>, Box<dyn Error + Sync + Send + 'static>>;
    fn create_customer(
        &self,
        customer: Customer,
    ) -> Result<Customer, Box<dyn Error + Sync + Send + 'static>>;
    fn edit_customer(
        &self,
        customer_ref: String,
        customer: Customer,
    ) -> Result<Customer, Box<dyn Error + Sync + Send + 'static>>;
    fn remove_customer<'a>(
        &self,
        customer_ref: &'a str,
    ) -> Result<(), Box<dyn Error + Sync + Send + 'static>>;
    fn create_settings(
        &self,
        settings: Settings,
    ) -> Result<(), Box<dyn Error + Sync + Send + 'static>>;
    fn edit_settings(
        &self,
        settings: Settings,
    ) -> Result<(), Box<dyn Error + Sync + Send + 'static>>;
    fn get_settings(&self) -> Result<Settings, Box<dyn Error + Sync + Send + 'static>>;

    fn generate_invoice<'a>(&self, invoice_path: &Path, output: &str) -> Result<PathBuf, Box<dyn Error + Sync + Send + 'static>>;
}
