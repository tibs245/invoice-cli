use std::error::Error;

use dialoguer::{Editor, Input};
use log::trace;

use crate::cli::context_parameters::ContextParameters;
use crate::entities::settings::{Enterprise, Settings};
use crate::entities::siren::Siren;
use crate::file_manager::file_manager::{FileManager, InvoiceManager};

pub fn edit_settings(context_parameters: ContextParameters) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    trace!("=== Edit settings");

    let file_manager = FileManager::new(
        context_parameters.invoice_manager_path,
        context_parameters.invoice_path,
        context_parameters.customer_file_path,
        context_parameters.config_file_path,
    )?;

    let settings: Settings = file_manager.get_settings()?;
    
    let name: String =
        Input::new().with_prompt("Enterprise name").with_initial_text(settings.enterprise.name).interact_text().unwrap();

    let title = Input::new().with_prompt("Job title").with_initial_text(settings.enterprise.title).interact_text().unwrap();

    let siren = loop {
        let siren_string: String =
            Input::new().with_prompt("SIREN").with_initial_text(settings.enterprise.siren.to_string()).interact_text().unwrap();

        match Siren::new(&siren_string) {
            Ok(some_siren) => break some_siren,
            Err(error) => println!("{}", error),
        };
    };

    let email = Input::new().with_prompt("Email").with_initial_text(settings.enterprise.email).interact_text().unwrap();

    let address = Input::new().with_prompt("Address").with_initial_text(settings.enterprise.address).interact_text().unwrap();

    let city = Input::new().with_prompt("City").with_initial_text(settings.enterprise.city).interact_text().unwrap();

    let postal = Input::new().with_prompt("Postal code").with_initial_text(settings.enterprise.postal).interact_text().unwrap();

    let phone = Input::new().with_prompt("Phone number").with_initial_text(settings.enterprise.phone).interact_text().unwrap();

    let politeness = Input::new()
        .with_prompt("Politeness")
        .with_initial_text(settings.politeness)
        .interact_text()
        .unwrap();

    let law_rules = Editor::new().edit(&settings.law_rules).unwrap().unwrap();

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

    file_manager.edit_settings(settings)?;

    Ok(())
}
