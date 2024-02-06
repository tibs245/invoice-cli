use crate::entities::settings::{Enterprise, Settings};
use crate::entities::siren::Siren;
use crate::file_manager::file_manager::{FileManager, Manager};
use dialoguer::{Editor, Input};
use log::{info, trace};
use std::error::Error;
use std::path::Path;

pub fn initiate_invoice_directory(
    invoice_manager_path: &Path,
    invoice_path: Option<&Path>,
    config_file_path: Option<&Path>,
    customer_file_path: Option<&Path>,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Initiate invoice directory");

    let name: String =
        Input::new().with_prompt("Enterprise name").interact_text().unwrap();

    let title = Input::new().with_prompt("Job title").interact_text().unwrap();

    let siren = loop {
        let siren_string: String =
            Input::new().with_prompt("SIREN").interact_text().unwrap();

        match Siren::new(&siren_string) {
            Ok(some_siren) => break some_siren,
            Err(error) => println!("{}", error),
        };
    };

    let email = Input::new().with_prompt("Email").interact_text().unwrap();

    let address = Input::new().with_prompt("Address").interact_text().unwrap();

    let city = Input::new().with_prompt("City").interact_text().unwrap();

    let postal = Input::new().with_prompt("Postal code").interact_text().unwrap();

    let phone = Input::new().with_prompt("Phone number").interact_text().unwrap();

    let politeness = Input::new()
        .with_prompt("Politeness")
        .with_initial_text("Thank you".to_string())
        .interact_text()
        .unwrap();

    let law_rules = Editor::new().edit("Payment Terms: Net 30 days from the invoice date. In accordance with the terms and conditions of sale, a late payment penalty of 40€ per month (or the maximum rate permitted by law, whichever is lower) will be applied to all overdue balances. Interest will accrue daily from the due date until full payment is received. In addition to the late payment penalty, the purchaser agrees to reimburse the seller for all costs incurred in collecting any late payments, including, but not limited to, legal fees and collection agency charges.").unwrap().unwrap();

    let settings = Settings {
        enterprise: Enterprise {
            name,
            title,
            siren,
            email,
            address,
            city,
            postal,
            phone,
        },
        law_rules,
        politeness,
    };

    let file_manager = FileManager::init(
        invoice_manager_path,
        invoice_path,
        customer_file_path,
        config_file_path,
    )?;
    file_manager.create_settings(settings)?;

    Ok(())
}
