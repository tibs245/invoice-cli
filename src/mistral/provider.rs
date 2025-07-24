use crate::entities::invoice::Invoice;
use crate::invoice_manager::invoice_manager::InvoiceManager;
use crate::mistral::api_response_models::ApiResponse;
use chrono::Local;
use log::debug;
use reqwest;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

fn generate_mistral_create_invoice_request(
    invoice_manager: &impl InvoiceManager,
    prompt: &str,
) -> Result<String, Box<dyn Error + Sync + Send + 'static>> {
    let all_customers: Vec<String> = invoice_manager
        .get_all_customers()?
        .into_iter()
        .map(|(id, _customer)| id)
        .collect();

    let current_date = Local::now().date_naive();
    let user_pref = invoice_manager
        .get_settings()?
        .llm_instruct
        .unwrap_or_else(|| "".to_string());

    let mut schema = serde_json::to_value(&schemars::schema_for!(Invoice))?;
    if let Some(desc) = schema
        .get_mut("properties")
        .and_then(|p| p.get_mut("customer_id"))
        .and_then(|v| v.as_object_mut())
    {
        desc.insert(
            "description".to_string(),
            json!(format!(
                "Customer ID. Must be a value of the this list: {}",
                all_customers.join(", ")
            )),
        );
    }

    let body = json!({
        "model": "ministral-8b-latest",
        "messages": [
            {
                "role": "system",
                "content": format!(
                    "Extract the invoice information to create it. If the date is not specified, use the default date: {} (in format: YYYY-MM-DD).",
                    current_date
                )
            },
            {"role": "system", "content": format!("The context and preference of the user : {}", user_pref)},
            {"role": "user", "content": prompt},
        ],
        "response_format": {
            "type": "json_schema",
            "json_schema": {
                "schema": schema,
                "name": "invoice",
                "strict": true
            }
        },
        "max_tokens": 256,
        "temperature": 0
    });

    Ok(body.to_string())
}

pub async fn extract_invoice_params(
    invoice_manager: &impl InvoiceManager,
    prompt: &str,
) -> Result<Invoice, Box<dyn Error + Sync + Send + 'static>> {
    let body = generate_mistral_create_invoice_request(invoice_manager, prompt)?;

    debug!("Request body: {}", body);

    let client = Client::new();
    let response = client
        .post("https://api.mistral.ai/v1/chat/completions")
        .bearer_auth(invoice_manager.get_settings()?.mistral_api_key.as_deref().unwrap_or(""))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
        debug!("Error response from server. Status: {}, Body: {}", status, text);
        Err(format!("Error response from server. Status: {}, Body: {}", status, text).into())
    } else {
        let json_response: ApiResponse = response.json().await?;
        debug!("Response LLM: {:?}", json_response);
        Ok(serde_json::from_str(&json_response.choices[0].message.content)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::customer::Customer;
    use crate::entities::product::Product;
    use crate::entities::settings::Settings;
    use crate::invoice_manager::invoice_manager::MockInvoiceManager;
    use chrono::Local;
    use std::collections::HashMap;

    fn get_hashmap_of_customers() -> HashMap<String, Customer> {
        let mut customers: HashMap<String, Customer> = HashMap::new();

        customers.insert(
            "John Doe LTD".to_string(),
            Customer {
                name: "John Doe".to_string(),
                address: "123 Main Street".to_string(),
                city: "Metropolis".to_string(),
                postal: "10001".to_string(),
            },
        );

        customers.insert(
            "Foo SAS".to_string(),
            Customer {
                name: "Jane Smith".to_string(),
                address: "456 Elm Street".to_string(),
                city: "Smallville".to_string(),
                postal: "54321".to_string(),
            },
        );

        customers
    }

    #[tokio::test]
    #[ignore]
    async fn test_extract_invoice_params() {
        let mut mock_file_manager = MockInvoiceManager::new();
        mock_file_manager
            .expect_get_all_customers()
            .returning(|| Ok(get_hashmap_of_customers()));
        mock_file_manager.expect_get_settings().returning(|| Ok(Settings::generate_simple_settings()));

        let prompt = "Create an invoice for John Doe for 3 'day of development' at 500 €";

        let result = extract_invoice_params(&mock_file_manager, prompt).await;

        let expected_invoice = Invoice {
            date: Local::now().date_naive(),
            customer_id: "John Doe LTD".to_string(),
            title: "".to_string(),
            invoice_day_id: None,
            products: vec![
                Product {
                    description: "Development".to_string(),
                    quantity: 3.0,
                    price: 500.0,
                }
            ],
        };

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.date, expected_invoice.date);
        assert_eq!(result.customer_id, expected_invoice.customer_id);
        assert_eq!(result.invoice_day_id, expected_invoice.invoice_day_id);
        assert_eq!(result.products.len(), 1);
        assert_eq!(result.products[0].price, expected_invoice.products[0].price);
        assert_eq!(result.products[0].quantity, expected_invoice.products[0].quantity);
    }
}
