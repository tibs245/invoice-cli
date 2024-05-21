use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use log::{error, info};

use crate::entities::customer::Customer;
use crate::entities::invoice::Invoice;
use crate::entities::settings::Settings;
use crate::file_manager::customer::create_customer::create_customer;
use crate::file_manager::customer::delete_customer::delete_customer;
use crate::file_manager::customer::edit_customer::edit_customer;
use crate::file_manager::customer::get_all_customers::get_all_customers;
use crate::file_manager::invoice::create_invoice::create_invoice;
use crate::file_manager::invoice::get_all_invoices::get_all_invoices;
use crate::file_manager::invoice::get_all_invoices_by_day::get_all_invoices_by_day;
use crate::file_manager::invoice::get_all_invoices_by_month::get_all_invoices_by_month;
use crate::file_manager::invoice::get_all_invoices_by_year::get_all_invoices_by_year;
use crate::file_manager::invoice::get_invoice_by_filepath::get_invoice_by_file_path;
use crate::file_manager::invoice_manager_error::InvoiceManagerError;
use crate::file_manager::settings::get_settings::get_settings;
use crate::file_manager::settings::save_settings::save_settings;

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
}

pub struct FileManager {
    invoice_path: PathBuf,
    customer_file_path: PathBuf,
    settings_file_path: PathBuf,
}

impl FileManager {
    const DEFAULT_INVOICE_PATH: &'static str = "invoices";
    const DEFAULT_CUSTOMER_FILE_PATH: &'static str = "customer.yaml";
    const DEFAULT_SETTINGS_FILE_PATH: &'static str = "settings.yaml";

    fn generate_instance(
        root_path: &Path,
        invoice_path: Option<&Path>,
        customer_file_path: Option<&Path>,
        settings_file_path: Option<&Path>,
    ) -> Result<Self, Box<dyn Error + Sync + Send + 'static>> {
        if !root_path.exists() && !root_path.parent().unwrap().exists() {
            error!(
                "Unable find parent of invoice directory in {}",
                root_path.to_string_lossy()
            );
            return Err(Box::try_from(InvoiceManagerError::UnableInitFolderInto(
                root_path.to_string_lossy().to_string(),
            ))
                .unwrap());
        }

        let invoice_path = match invoice_path {
            Some(invoice_path_given) => invoice_path_given.to_owned(),
            None => root_path.to_owned().join(Self::DEFAULT_INVOICE_PATH),
        };

        let customer_file_path = match customer_file_path {
            Some(customer_path_given) => customer_path_given.to_owned(),
            None => root_path.to_owned().join(Self::DEFAULT_CUSTOMER_FILE_PATH),
        };

        let settings_file_path = match settings_file_path {
            Some(settings_path_given) => settings_path_given.to_owned(),
            None => root_path.to_owned().join(Self::DEFAULT_SETTINGS_FILE_PATH),
        };

        Ok(FileManager {
            invoice_path,
            customer_file_path,
            settings_file_path,
        })
    }
    pub fn new(
        root_path: &Path,
        invoice_path: Option<&Path>,
        customer_file_path: Option<&Path>,
        settings_file_path: Option<&Path>,
    ) -> Result<Self, Box<dyn Error + Sync + Send + 'static>> {
        let file_manager = Self::generate_instance(
            root_path,
            invoice_path,
            customer_file_path,
            settings_file_path,
        )?;

        if !root_path.exists() && root_path.parent().unwrap().exists() {
            error!(
                "Unable access Invoice directory{}\n Maybe Init before use",
                root_path.to_string_lossy()
            );
            return Err(Box::try_from(InvoiceManagerError::InvoiceStorePathNotFound(
                root_path.to_string_lossy().to_string(),
            ))
                .unwrap());
        }

        if !file_manager.invoice_path.is_dir() {
            error!(
                "Unable access Invoice directory {}\n Maybe Init before use",
                file_manager.invoice_path.to_string_lossy()
            );

            return Err(Box::try_from(InvoiceManagerError::InvoiceStorePathNotFound(
                file_manager.invoice_path.to_string_lossy().to_string(),
            ))
                .unwrap());
        }

        if !file_manager.customer_file_path.exists() {
            error!(
                "Customer file not exists : {}\n Maybe Init before use",
                file_manager.customer_file_path.to_string_lossy()
            );

            return Err(
                Box::try_from(InvoiceManagerError::CustomerStorePathNotFound(
                    file_manager.customer_file_path.to_string_lossy().to_string(),
                ))
                    .unwrap(),
            );
        }

        Ok(file_manager)
    }

    pub fn init(
        root_path: &Path,
        invoice_path: Option<&Path>,
        customer_file_path: Option<&Path>,
        settings_file_path: Option<&Path>,
    ) -> Result<Self, Box<dyn Error + Sync + Send + 'static>> {
        let file_manager = Self::generate_instance(
            root_path,
            invoice_path,
            customer_file_path,
            settings_file_path,
        )?;

        if !root_path.exists() && root_path.parent().unwrap().exists() {
            info!("Create root directory in {}", root_path.to_string_lossy());
            if let Err(error) = fs::create_dir(root_path) {
                error!(
                    "Unable create root directory in {}",
                    root_path.to_string_lossy()
                );
                return Err(Box::try_from(InvoiceManagerError::UnableToCreateDirectory(
                    root_path.to_string_lossy().to_string(),
                    error,
                ))
                    .unwrap());
            }
        }

        if !&file_manager.invoice_path.is_dir() {
            info!(
                "Create invoice directory in {}",
                &file_manager.invoice_path.to_string_lossy()
            );
            if let Err(error) = fs::create_dir(&file_manager.invoice_path.clone()) {
                error!(
                    "Unable create invoice directory in {}",
                    root_path.to_string_lossy()
                );
                return Err(Box::try_from(InvoiceManagerError::UnableToCreateDirectory(
                    file_manager.invoice_path.to_string_lossy().to_string(),
                    error,
                ))
                    .unwrap());
            }
        }

        if !&file_manager.customer_file_path.exists() {
            info!(
                "Create customer file in {}",
                &file_manager.customer_file_path.to_string_lossy()
            );
            if let Err(error) = fs::write(&file_manager.customer_file_path, "") {
                error!(
                    "Unable to create customer file in {}",
                    root_path.to_string_lossy()
                );
                return Err(Box::try_from(
                    InvoiceManagerError::UnableToWriteCustomerFile(
                        file_manager.customer_file_path.to_string_lossy().to_string(),
                        error,
                    ),
                )
                    .unwrap());
            }
        }

        if !&file_manager.settings_file_path.exists() {
            info!(
                "Create settings file in {}",
                &file_manager.settings_file_path.to_string_lossy()
            );
            if let Err(error) = fs::write(&file_manager.settings_file_path, "") {
                error!(
                    "Unable to create settings file in {}",
                    root_path.to_string_lossy()
                );
                return Err(Box::try_from(
                    InvoiceManagerError::UnableToWriteCustomerFile(
                        file_manager.settings_file_path.to_string_lossy().to_string(),
                        error,
                    ),
                )
                    .unwrap());
            }
        }

        Ok(file_manager)
    }
}

impl InvoiceManager for FileManager {
    fn create_invoice(
        &self,
        invoice: Invoice,
    ) -> Result<PathBuf, Box<dyn Error + Sync + Send + 'static>> {
        create_invoice(self.invoice_path.as_path(), invoice)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }
    fn get_all_invoices(
        &self,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>> {
        get_all_invoices(self.invoice_path.as_path())
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }
    fn get_invoice_by_ref(
        &self,
        invoice_reference: &str,
    ) -> Result<Invoice, Box<dyn Error + Sync + Send + 'static>> {
        get_invoice_by_file_path(
            &self
                .invoice_path
                .as_path()
                .join(invoice_reference.to_string() + ".yaml"),
        )
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }
    fn get_invoice_by_date(
        &self,
        day: NaiveDate,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>> {
        get_all_invoices_by_day(self.invoice_path.as_path(), day)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn get_invoice_by_month(
        &self,
        year: i32,
        month: u32,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>> {
        get_all_invoices_by_month(self.invoice_path.as_path(), year, month)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn get_invoice_by_year(
        &self,
        year: i32,
    ) -> Result<Vec<Invoice>, Box<dyn Error + Sync + Send + 'static>> {
        get_all_invoices_by_year(self.invoice_path.as_path(), year)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn get_all_customers(
        &self,
    ) -> Result<HashMap<String, Customer>, Box<dyn Error + Sync + Send + 'static>> {
        get_all_customers(self.customer_file_path.as_path())
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn create_customer(
        &self,
        customer: Customer,
    ) -> Result<Customer, Box<dyn Error + Sync + Send + 'static>> {
        create_customer(self.customer_file_path.as_path(), customer)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn edit_customer(
        &self,
        customer_ref: String,
        customer: Customer,
    ) -> Result<Customer, Box<dyn Error + Sync + Send + 'static>> {
        edit_customer(self.customer_file_path.as_path(), customer_ref, customer)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn remove_customer(
        &self,
        customer_ref: &str,
    ) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
        delete_customer(self.customer_file_path.as_path(), customer_ref)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn create_settings(
        &self,
        settings: Settings,
    ) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
        save_settings(&self.settings_file_path, &settings)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn edit_settings(
        &self,
        settings: Settings,
    ) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
        save_settings(&self.settings_file_path, &settings)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }

    fn get_settings(&self) -> Result<Settings, Box<dyn Error + Sync + Send + 'static>> {
        get_settings(&self.settings_file_path)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send + 'static>)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::entities::invoice::InvoiceDayId;
    use crate::entities::product::Product;

    use super::*;

    #[test]
    pub fn file_manager_generate_instance() {
        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();
        let temp_dir = temp_dir_assert_fs.path();
        let file_manager = FileManager::generate_instance(temp_dir, None, None, None)
            .expect("Unable initiate file manager");

        assert_eq!(
            file_manager.invoice_path,
            temp_dir.to_owned().join(FileManager::DEFAULT_INVOICE_PATH)
        );
        assert_eq!(
            file_manager.customer_file_path,
            temp_dir.to_owned().join(FileManager::DEFAULT_CUSTOMER_FILE_PATH)
        );
        assert_eq!(
            file_manager.settings_file_path,
            temp_dir.to_owned().join(FileManager::DEFAULT_SETTINGS_FILE_PATH)
        );

        let file_manager = FileManager::generate_instance(
            &temp_dir,
            Some(&(temp_dir.to_owned().join("custom_invoice_folder"))),
            Some(&(temp_dir.to_owned().join("custom_enterprise"))),
            Some(&(temp_dir.to_owned().join("custom_settings"))),
        )
            .expect("Unable initiate file manager");

        assert_eq!(
            file_manager.invoice_path,
            temp_dir.to_owned().join("custom_invoice_folder")
        );
        assert_eq!(
            file_manager.customer_file_path,
            temp_dir.to_owned().join("custom_enterprise")
        );
        assert_eq!(
            file_manager.settings_file_path,
            temp_dir.to_owned().join("custom_settings")
        );

        temp_dir_assert_fs.close().unwrap();
    }

    #[test]
    pub fn test_file_manager_create_read() {
        fn test_all_invoices_data_for_manager(file_manager: &FileManager) {
            let all_invoices = file_manager.get_all_invoices();

            assert!(all_invoices.is_ok());

            let all_invoices_path = all_invoices.unwrap();
            assert_eq!(all_invoices_path.len(), 5);
        }

        fn test_invoice_result_data_for_manager(file_manager: &FileManager) {
            let invoice_result = file_manager.get_invoice_by_ref("2015031402");

            assert!(invoice_result.is_ok());

            let invoice = invoice_result.unwrap();
            assert_eq!(
                invoice.invoice_day_id,
                Some(InvoiceDayId::new("02").unwrap())
            );
            assert_eq!(invoice.title, "Test invoice for simple customer");
        }

        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();

        let file_manager = FileManager::new(temp_dir_assert_fs.path(), None, None, None);

        // Test if we have error on new instance before init
        assert!(file_manager.is_err());

        let file_manager = FileManager::init(temp_dir_assert_fs.path(), None, None, None)
            .expect("Unable initiate file manager");

        assert!(temp_dir_assert_fs.join(FileManager::DEFAULT_INVOICE_PATH).exists());
        assert!(temp_dir_assert_fs
            .join(FileManager::DEFAULT_CUSTOMER_FILE_PATH)
            .exists());
        assert!(temp_dir_assert_fs
            .join(FileManager::DEFAULT_SETTINGS_FILE_PATH)
            .exists());

        for index in 1..=5 {
            let simple_product = Product {
                description: "Product example".to_string(),
                quantity: index as f32,
                price: 350.0,
            };

            let invoice_example = Invoice {
                invoice_day_id: Some(InvoiceDayId::new(&index.to_string()).unwrap()),
                date: NaiveDate::from_ymd_opt(2015, 3, 14).unwrap(),
                customer_id: "king".to_string(),
                title: "Test invoice for simple customer".to_string(),
                products: vec![simple_product],
            };

            file_manager
                .create_invoice(invoice_example)
                .expect("Unable create invoice in temp dir");
        }

        test_all_invoices_data_for_manager(&file_manager);
        test_invoice_result_data_for_manager(&file_manager);

        // Test prevent if we have not lost data with regeneration
        let file_manager = FileManager::init(temp_dir_assert_fs.path(), None, None, None)
            .expect("Unable initiate file manager");

        test_all_invoices_data_for_manager(&file_manager);
        test_invoice_result_data_for_manager(&file_manager);

        // Test new instance work
        let file_manager = FileManager::new(temp_dir_assert_fs.path(), None, None, None)
            .expect("Unable create new file manager instance");

        test_all_invoices_data_for_manager(&file_manager);
        test_invoice_result_data_for_manager(&file_manager);

        temp_dir_assert_fs.close().unwrap();
    }
}
