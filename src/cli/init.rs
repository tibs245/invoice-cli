use std::error::Error;

use dialoguer::{Editor, Input};
use log::trace;

use crate::entities::settings::{Enterprise, Settings};
use crate::entities::siren::Siren;
use crate::file_manager::context_parameters::ContextParameters;
use crate::file_manager::file_manager::FileManager;
use crate::invoice_manager::invoice_manager::InvoiceManager;

pub fn initiate_invoice_directory(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
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
    
    let tva = Input::new().with_prompt("TVA Number").allow_empty(true).interact_text().unwrap();

    let politeness = Input::new()
        .with_prompt("Politeness")
        .with_initial_text("Thank you".to_string())
        .interact_text()
        .unwrap();

    let law_rules = Editor::new().edit("Payment Terms: Net 30 days from the invoice date. In accordance with the terms and conditions of sale, a late payment penalty of 40€ per month (or the maximum rate permitted by law, whichever is lower) will be applied to all overdue balances. Interest will accrue daily from the due date until full payment is received. In addition to the late payment penalty, the purchaser agrees to reimburse the seller for all costs incurred in collecting any late payments, including, but not limited to, legal fees and collection agency charges.").unwrap().unwrap();

    let llm_instruct: String = Input::new().with_prompt("LLM Instruction").allow_empty(true).interact_text().unwrap();
    let llm_instruct = if &llm_instruct == "" { None } else { Some(llm_instruct) };

    let mistral_api_key = Input::new().with_prompt("Mistral API Key").allow_empty(true).interact_text().unwrap();
    let mistral_api_key = if &mistral_api_key == "" { None } else { Some(mistral_api_key) };
    
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
            tva
        },
        law_rules,
        politeness,
        llm_instruct,
        mistral_api_key,
    };

    let file_manager = FileManager::init(context_parameters)?;
    file_manager.create_settings(settings)?;

    Ok(())
}
