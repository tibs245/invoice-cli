use std::error::Error;

use chrono::Local;
use dialoguer::{Confirm, FuzzySelect, Input};
use log::trace;

use crate::entities::invoice::Invoice;
use crate::entities::product::Product;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn create_invoice(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Create invoice");

    let file_manager = FileManager::new(context_parameters)?;

    let all_customers_id: Vec<String> = file_manager
        .get_all_customers()?
        .into_iter()
        .map(|(id, _customer)| id)
        .collect();

    let date = Local::now().date_naive();

    let customer_index = FuzzySelect::new()
        .with_prompt("What is your customer?")
        .items(&all_customers_id)
        .interact()
        .unwrap();

    let title = Input::new().with_prompt("Invoice title").interact_text().unwrap();

    let mut products = vec![];

    loop {
        let product_title: String =
            Input::new().with_prompt("Product title").interact_text().unwrap();

        if product_title == "".to_string() {
            break;
        }

        let product_quantity = Input::new()
            .with_prompt("Product quantity")
            .validate_with(|input: &String| -> Result<(), &str> {
                input.parse::<f32>().map_err(|_| "Invalid number")?;
                return Ok(());
            })
            .interact()
            .unwrap();

        let product_price = Input::new()
            .with_prompt("Product price")
            .validate_with(|input: &String| -> Result<(), &str> {
                input.parse::<f32>().map_err(|_| "Invalid number")?;
                return Ok(());
            })
            .interact()
            .unwrap();

        products.push(Product {
            description: product_title,
            quantity: product_quantity.parse::<f32>().unwrap(),
            price: product_price.parse::<f32>().unwrap(),
        });

        let confirmation = Confirm::new()
            .with_prompt("Do you want to add another product ?")
            .interact()
            .unwrap();

        if !confirmation {
            break;
        }
    }

    let invoice = Invoice {
        date,
        customer_id: all_customers_id[customer_index].to_owned(),
        title,
        products,
        invoice_day_id: None,
    };

    let invoice_path = file_manager.create_invoice(invoice)?;

    println!("Invoice created in : {}", invoice_path.to_string_lossy());
    
    let mut invoice_path_output = invoice_path.clone();
    invoice_path_output.set_extension("pdf");

    file_manager.generate_invoice(invoice_path.as_path(), invoice_path_output.file_name().unwrap().to_str().unwrap())?;

    Ok(())
}
