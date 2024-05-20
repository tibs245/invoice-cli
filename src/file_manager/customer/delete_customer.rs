use std::fs;
use std::path::Path;

use super::get_all_customers::get_all_customers;
use super::invoice_customer_manager_error::InvoiceCustomerManagerError;

pub fn delete_customer(
    customer_file_path: &Path,
    customer_ref: &str,
) -> Result<(), InvoiceCustomerManagerError> {
    if !customer_file_path.is_file() {
        return Err(InvoiceCustomerManagerError::CustomerStorePathNotFound(
            customer_file_path.to_string_lossy().to_string(),
        ));
    }

    let mut all_customers = get_all_customers(customer_file_path)?;

    if !all_customers.contains_key(customer_ref) {
        return Err(
            InvoiceCustomerManagerError::UnableDeleteCustomerRefNotFound(
                customer_ref.to_owned(),
            ),
        );
    }

    all_customers.remove(customer_ref);

    match fs::write(
        customer_file_path,
        serde_yaml::to_string(&all_customers).unwrap(),
    ) {
        Ok(()) => Ok(()),
        Err(error) => Err(InvoiceCustomerManagerError::UnableToWriteCustomerFile(
            customer_file_path.to_string_lossy().to_string(),
            error,
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::file_manager::customer::delete_customer::delete_customer;
    use crate::file_manager::customer::get_all_customers::get_all_customers;

    #[test]
    pub fn test_delete_customer() {
        let customer_map_yaml = "king:\n".to_owned()
            + "  name: King SARL\n"
            + "  address: 1 rue des champs\n"
            + "  city: Paris\n"
            + "  postal: '75000'\n"
            + "last_king:\n"
            + "  name: Last King SARL\n"
            + "  address: 2 rue des champs\n"
            + "  city: Paris\n"
            + "  postal: '75000'\n";

        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();
        let temp_customer_file_path = temp_dir_assert_fs.path().join("customer.yaml");

        fs::write(&temp_customer_file_path, &customer_map_yaml)
            .expect("Unable to delete king in customer file");

        delete_customer(&temp_customer_file_path, "king").unwrap();

        let all_customer = get_all_customers(&temp_customer_file_path)
            .expect("Unable read customer created");

        assert_eq!(all_customer.len(), 1);
        assert!(!all_customer.contains_key("king"));
        assert!(all_customer.contains_key("last_king"));

        delete_customer(&temp_customer_file_path, "last_king")
            .expect("Unable to delete the second customer");

        let all_customer = get_all_customers(&temp_customer_file_path)
            .expect("Unable read customer created");

        assert_eq!(all_customer.len(), 0);
        assert!(!all_customer.contains_key("king"));
        assert!(!all_customer.contains_key("last_king"));

        let error_customer_not_exist = delete_customer(&temp_customer_file_path, "king");

        assert!(error_customer_not_exist.is_err());
        
        temp_dir_assert_fs.close().unwrap();
    }
}
