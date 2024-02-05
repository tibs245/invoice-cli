use super::get_all_customers::get_all_customers;
use super::invoice_customer_manager_error::InvoiceCustomerManagerError;
use crate::entities::customer::Customer;
use std::fs;
use std::path::Path;

pub fn create_customer(
    customer_file_path: &Path,
    customer: Customer,
) -> Result<Customer, InvoiceCustomerManagerError> {
    if !customer_file_path.is_file() {
        return Err(InvoiceCustomerManagerError::CustomerStorePathNotFound(
            customer_file_path.to_string_lossy().to_string(),
        ));
    }

    let mut all_customers = get_all_customers(customer_file_path)?;

    if all_customers.contains_key(&customer.serialized_name()) {
        return Err(
            InvoiceCustomerManagerError::UnableCreateCustomerDuplicatedId(
                customer.serialized_name(),
                customer.serialized_name(),
            ),
        );
    }

    all_customers.insert(customer.serialized_name(), customer.clone());

    match fs::write(
        customer_file_path,
        serde_yaml::to_string(&all_customers).unwrap(),
    ) {
        Ok(()) => Ok(customer),
        Err(error) => Err(InvoiceCustomerManagerError::UnableToWriteCustomerFile(
            customer_file_path.to_string_lossy().to_string(),
            error,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::customer::Customer;
    use crate::file_manager::customer::create_customer::create_customer;
    use crate::file_manager::customer::get_all_customers::get_all_customers;
    use std::fs;

    #[test]
    pub fn test_create_customer() {
        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();
        let temp_customer_file_path = temp_dir_assert_fs.path().join("customer.yaml");

        fs::write(&temp_customer_file_path, "")
            .expect("Unable to create empty customer file");

        let first_customer = Customer::simple_with_name("First".to_string());

        create_customer(&temp_customer_file_path, first_customer).unwrap();

        let all_customer = get_all_customers(&temp_customer_file_path)
            .expect("Unable read customer created");

        assert_eq!(all_customer.len(), 1);
        assert!(all_customer.contains_key("first"));

        let second_customer = Customer::simple_with_name("Second".to_string());

        create_customer(&temp_customer_file_path, second_customer)
            .expect("Unable to create the second customer");

        let all_customer = get_all_customers(&temp_customer_file_path)
            .expect("Unable read customer created");

        assert_eq!(all_customer.len(), 2);
        assert!(all_customer.contains_key("first"));
        assert!(all_customer.contains_key("second"));

        temp_dir_assert_fs.close().unwrap();
    }
}
