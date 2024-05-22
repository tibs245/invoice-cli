use std::collections::HashMap;
use std::error::Error;

use dialoguer::FuzzySelect;

use crate::entities::customer::Customer;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub(crate) fn select_customer(file_manager: &FileManager) -> Result<(String, Customer), Box<dyn Error + Sync + Send + 'static>> {
    let all_customers_hashmap: HashMap<String, Customer> = file_manager.get_all_customers()?;
    let all_customers: Vec<(&String, &Customer)> = all_customers_hashmap.iter().collect();

    let customer_index = FuzzySelect::new()
        .with_prompt("What is your customer?")
        .items(&all_customers.iter().map(|(_customer_ref, customer)| customer.name.clone()).collect::<Vec<String>>())
        .interact()
        .unwrap();

    return Ok((all_customers[customer_index].0.to_string(), all_customers[customer_index].1.clone()));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::error::Error;

    use crate::cli::utils::cli_utils_error::CliUtilsError;
    use crate::entities::customer::Customer;
    use crate::invoice_manager::invoice_manager::InvoiceManager;

    pub(crate) fn mock_select_customer(file_manager: &impl InvoiceManager) -> Result<Customer, Box<dyn Error + Sync + Send + 'static>> {
        let all_customers: HashMap<String, Customer> = file_manager.get_all_customers()?;

        if all_customers.len() == 0 {
            return Err(Box::new(CliUtilsError::NoInvoiceFound()));
        }

        Ok(all_customers.values().next().unwrap().clone())
    }
}