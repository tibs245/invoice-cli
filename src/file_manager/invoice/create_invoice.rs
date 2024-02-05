use super::get_all_invoices_by_day::get_all_invoices_by_day;
use super::invoice_file_manager_error::InvoiceFileManagerError;
use crate::entities::invoice::{Invoice, InvoiceDayId};
use std::fs;
use std::path::{Path, PathBuf};

pub fn assign_next_id_to_invoice(path: &Path, invoice: Invoice) -> Invoice {
    let last_id = get_all_invoices_by_day(path, invoice.date)
        .unwrap()
        .iter()
        .map(|invoice| {
            invoice
                .invoice_day_id
                .clone()
                .unwrap()
                .to_string()
                .parse::<u8>()
                .map_err(|_| "Invalid number id".to_string())
                .unwrap_or(0)
        })
        .max();

    let invoice_day_id = match last_id {
        Some(last_id) => InvoiceDayId::new(&(last_id + 1).to_string()).unwrap(),
        None => InvoiceDayId::new("01").unwrap(),
    };

    Invoice {
        invoice_day_id: Some(InvoiceDayId::new(&invoice_day_id.to_string()).unwrap()),
        ..invoice
    }
}

pub fn create_invoice(
    path: &Path,
    invoice: Invoice,
) -> Result<PathBuf, InvoiceFileManagerError> {
    if !path.is_dir() {
        return Err(InvoiceFileManagerError::InvoiceStorePathNotFound(
            path.to_string_lossy().to_string(),
        ));
    }

    let invoice = match invoice.invoice_day_id {
        Some(_) => invoice,
        None => assign_next_id_to_invoice(path, invoice),
    };

    let file_path = path.to_owned().join(invoice.get_ref().unwrap() + ".yaml");

    match fs::write(file_path.clone(), serde_yaml::to_string(&invoice).unwrap()) {
        Ok(()) => Ok(file_path.as_path().to_owned()),
        Err(error) => Err(InvoiceFileManagerError::UnableToWriteInvoiceFile(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::invoice::InvoiceDayId;
    use crate::entities::product::Product;
    use chrono::NaiveDate;
    use std::env;

    #[test]
    pub fn test_create_invoice() {
        let temp_dir = env::temp_dir()
            .as_path()
            .join("rust_unit_test_invoice_cli_test_create_invoice");

        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir.clone()).expect("Unable remove temp dir folder");
        }

        fs::create_dir(temp_dir.clone()).expect("Unable create temp dir to test");

        let simple_product = Product {
            description: "Product example".to_string(),
            quantity: 1.0,
            price: 350.0,
        };

        let invoice_example = Invoice {
            invoice_day_id: Some(InvoiceDayId::new("02").unwrap()),
            date: NaiveDate::from_ymd_opt(2020, 3, 14).unwrap(),
            customer_id: "king".to_string(),
            title: "Test invoice reference".to_string(),
            products: vec![simple_product],
        };

        let invoice_created = create_invoice(&temp_dir, invoice_example);
        assert!(invoice_created.is_ok());

        let invoice_path = invoice_created.unwrap();
        assert_eq!(
            invoice_path,
            temp_dir.clone().join("2020031402.yaml").as_path()
        );
        assert!(invoice_path.exists());

        fs::remove_dir_all(temp_dir).expect("Unable remove temp dir folder");
    }

    #[test]
    pub fn test_assign_invoice_id() {
        let temp_dir = env::temp_dir()
            .as_path()
            .join("rust_unit_test_invoice_cli_test_assign_invoice_id");

        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir.clone()).expect("Unable remove temp dir folder");
        }

        fs::create_dir(temp_dir.clone()).expect("Unable create temp dir to test");

        [
            Invoice::generate_simple_invoice_with_id_and_date_example(
                Some(InvoiceDayId::new("01").unwrap()),
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            ),
            Invoice::generate_simple_invoice_with_id_and_date_example(
                Some(InvoiceDayId::new("01").unwrap()),
                NaiveDate::from_ymd_opt(2020, 2, 1).unwrap(),
            ),
            Invoice::generate_simple_invoice_with_id_and_date_example(
                Some(InvoiceDayId::new("04").unwrap()),
                NaiveDate::from_ymd_opt(2020, 3, 1).unwrap(),
            ),
            Invoice::generate_simple_invoice_with_id_and_date_example(
                Some(InvoiceDayId::new("02").unwrap()),
                NaiveDate::from_ymd_opt(2020, 3, 1).unwrap(),
            ),
            Invoice::generate_simple_invoice_with_id_and_date_example(
                Some(InvoiceDayId::new("01").unwrap()),
                NaiveDate::from_ymd_opt(2021, 3, 1).unwrap(),
            ),
        ]
        .into_iter()
        .for_each(|invoice| {
            create_invoice(temp_dir.as_path(), invoice)
                .expect("Unable create test invoice");
        });

        fs::write(temp_dir.clone().join(".gitignore"), "Just a unit test")
            .expect("Unable to write in temporary directory to test");

        let invoice_example = Invoice::generate_simple_invoice_with_id_and_date_example(
            None,
            NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
        );

        let new_invoice = assign_next_id_to_invoice(temp_dir.as_path(), invoice_example);

        assert_eq!(
            new_invoice.invoice_day_id,
            Some(InvoiceDayId::new("01").unwrap()),
            "Unable have first ID when we have not invoice on the day"
        );

        let invoice_example = Invoice::generate_simple_invoice_with_id_and_date_example(
            None,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        let new_invoice = assign_next_id_to_invoice(temp_dir.as_path(), invoice_example);

        assert_eq!(
            new_invoice.invoice_day_id,
            Some(InvoiceDayId::new("02").unwrap()),
            "Unable have second ID when we have one invoice on the day"
        );

        let invoice_example = Invoice::generate_simple_invoice_with_id_and_date_example(
            None,
            NaiveDate::from_ymd_opt(2020, 3, 1).unwrap(),
        );

        let new_invoice = assign_next_id_to_invoice(temp_dir.as_path(), invoice_example);

        assert_eq!(
            new_invoice.invoice_day_id,
            Some(InvoiceDayId::new("05").unwrap()),
            "Unable have multiple ID when we have multiple invoices on the day"
        );
    }
}
