use super::get_all_invoices_path::get_all_invoices_path;
use super::get_invoice_by_filepath::get_invoice_by_file_path;
use super::invoice_file_manager_error::InvoiceFileManagerError;
use crate::entities::invoice::Invoice;
use std::path::{Path, PathBuf};

pub fn get_all_invoices_by_year(
    path: &Path,
    year: i32,
) -> Result<Vec<Invoice>, InvoiceFileManagerError> {
    get_all_invoices_path(path)
        .unwrap()
        .iter()
        .filter(|file_path| {
            file_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(&year.to_string())
        })
        .map(|file_path: &PathBuf| get_invoice_by_file_path(file_path))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    pub fn test_get_all_invoices_path() {
        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();
        let temp_dir = temp_dir_assert_fs.path();

        [
            "2020010101.yaml".to_string(),
            "2020030101.yaml".to_string(),
            ".gitignore".to_string(),
            "2020030102.yaml".to_string(),
            "2021031001.yaml".to_string(),
        ]
        .iter()
        .for_each(|filename| {
            fs::write(
                temp_dir.join(filename),
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
        });

        let all_invoices_path_result = get_all_invoices_by_year(temp_dir, 2020);

        assert!(all_invoices_path_result.is_ok());

        let all_invoices_path = all_invoices_path_result.unwrap();
        assert_eq!(all_invoices_path.len(), 3);

        assert!(all_invoices_path
            .iter()
            .any(|product| product.title == "Test invoice for simple customer"));

        temp_dir_assert_fs.close().unwrap()
    }
}
