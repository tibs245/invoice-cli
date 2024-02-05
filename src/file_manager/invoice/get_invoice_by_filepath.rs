use super::invoice_file_manager_error::InvoiceFileManagerError;
use crate::entities::invoice::Invoice;
use std::fs;
use std::path::PathBuf;

pub fn get_invoice_by_file_path(
    file_path: &PathBuf,
) -> Result<Invoice, InvoiceFileManagerError> {
    match fs::read_to_string(file_path) {
        Ok(invoice_data) => {
            let invoice_example: Invoice = serde_yaml::from_str(&invoice_data).unwrap();
            Ok(invoice_example)
        }
        Err(e) => Err(InvoiceFileManagerError::UnableToReadPath(
            file_path.to_string_lossy().to_string(),
            e,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::invoice::InvoiceDayId;
    use chrono::NaiveDate;
    use std::env;

    #[test]
    pub fn test_get_invoice() {
        let temp_dir = env::temp_dir()
            .as_path()
            .join("rust_unit_test_invoice_cli_test_get_all_invoices_name");

        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir.clone()).expect("Unable remove temp dir folder");
        }

        fs::create_dir(temp_dir.clone()).expect("Unable create temp dir to test");

        let file_path = temp_dir.clone().join("2020010101.yaml");

        fs::write(
            file_path.clone(),
            "date:\n".to_owned()
                + "  day: '14'\n"
                + "  month: '03'\n"
                + "  year: '2010'\n"
                + "customer_id: king\n"
                + "title: Test invoice for simple customer\n"
                + "invoice_day_id: '01'\n"
                + "products:\n"
                + "- description: Product example\n"
                + "  quantity: 1.0\n"
                + "  price: 350.0\n",
        )
        .expect("Unable to write in temporary directory to test");

        let invoice_data: Invoice = get_invoice_by_file_path(&file_path).unwrap();

        assert_eq!(
            invoice_data.invoice_day_id,
            Some(InvoiceDayId::new("01").unwrap())
        );
        assert_eq!(
            invoice_data.date,
            NaiveDate::from_ymd_opt(2010, 3, 14).expect("Date de test incorrect")
        );
        assert_eq!(invoice_data.title, "Test invoice for simple customer");
        assert_eq!(invoice_data.products.len(), 1);
        assert_eq!(invoice_data.get_total_price(), 350.0);

        fs::remove_dir_all(temp_dir).expect("Unable remove temp dir folder");
    }
}
