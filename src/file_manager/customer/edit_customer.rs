use std::fs;
use std::path::Path;

use crate::entities::customer::Customer;

use super::get_all_customers::get_all_customers;
use super::invoice_customer_manager_error::InvoiceCustomerManagerError;

pub fn edit_customer(
    customer_file_path: &Path,
    customer_ref: String,
    customer: Customer,
) -> Result<Customer, InvoiceCustomerManagerError> {
    if !customer_file_path.is_file() {
        return Err(InvoiceCustomerManagerError::CustomerStorePathNotFound(
            customer_file_path.to_string_lossy().to_string(),
        ));
    }

    let mut all_customers = get_all_customers(customer_file_path)?;

    if !all_customers.contains_key(&customer_ref) {
        return Err(
            InvoiceCustomerManagerError::UnableEditCustomerDuplicatedId(
                customer.serialized_name(),
            ),
        );
    }

    all_customers.insert(customer_ref, customer.clone());

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
    use std::fs;

    use crate::entities::customer::Customer;
    use crate::file_manager::customer::edit_customer::edit_customer;
    use crate::file_manager::customer::get_all_customers::get_all_customers;

    #[test]
    pub fn test_edit_customer_not_exists() {
        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();
        let temp_customer_file_path = temp_dir_assert_fs.path().join("customer.yaml");

        fs::write(&temp_customer_file_path, "")
            .expect("Unable to create empty customer file");

        let first_customer = Customer::simple_with_name("First".to_string());

        let result = edit_customer(&temp_customer_file_path, "first".to_string(), first_customer);

        assert!(result.is_err());
    }


    #[test]
    pub fn test_edit_customer() {
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
            .expect("Unable to create empty customer file");

        let first_customer = Customer::simple_with_name("king".to_string());

        edit_customer(&temp_customer_file_path, "king".to_string(), first_customer.clone()).unwrap();

        let all_customer = get_all_customers(&temp_customer_file_path)
            .expect("Unable read customer edited");
        
        let king_customer = all_customer.get("king").unwrap();
        assert_eq!(king_customer.name, first_customer.name);
        assert_eq!(king_customer.address, first_customer.address);
        assert_eq!(king_customer.city, first_customer.city);
        assert_eq!(king_customer.postal, first_customer.postal);
    }
}
