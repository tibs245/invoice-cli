use super::invoice_customer_manager_error::InvoiceCustomerManagerError;
use crate::entities::customer::Customer;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn get_all_customers(
    customer_file_path: &Path,
) -> Result<HashMap<String, Customer>, InvoiceCustomerManagerError> {
    match fs::read_to_string(customer_file_path) {
        Ok(invoice_data) => {
            let customer: HashMap<String, Customer> =
                serde_yaml::from_str(&invoice_data).unwrap();
            Ok(customer)
        }
        Err(e) => Err(InvoiceCustomerManagerError::UnableToReadPath(
            customer_file_path.to_string_lossy().to_string(),
            e,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::file_manager::customer::get_all_customers::get_all_customers;
    use assert_fs::prelude::{FileWriteStr, PathChild};

    #[test]
    fn customer_map_from_string() {
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

        let temp_dir = assert_fs::TempDir::new().unwrap();
        let temp_customer_file = temp_dir.child("customer.yaml");

        temp_customer_file
            .write_str(&customer_map_yaml)
            .expect("Cannot write temporary file");

        let all_customers = get_all_customers(temp_customer_file.path())
            .expect("Unable to read customer file example");

        assert!(all_customers.contains_key("king"));
        assert!(all_customers.contains_key("last_king"));
        assert_eq!(all_customers.len(), 2);

        temp_dir.close().unwrap();
    }
}
