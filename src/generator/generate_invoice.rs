use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use log::{error, info};

use crate::generator::generator_error::GeneratorError;

fn generate_default_template() -> String {
    include_str!("assets/default_template.typ").to_string()
}

fn generate_main_template(settings_path: &Path, customer_path: &Path, invoice_path: &Path, template_path: &Path) -> String {
    let main_template = include_str!("assets/main_template.typ");

    let main_template = main_template.replace("{{ TEMPLATE_PATH }}", template_path.to_str().unwrap());
    let main_template = main_template.replace("{{ SETTINGS_PATH }}", settings_path.to_str().unwrap());
    let main_template = main_template.replace("{{ CUSTOMERS_PATH }}", customer_path.to_str().unwrap());
    main_template.replace("{{ INVOICE_PATH }}", invoice_path.to_str().unwrap())
}

pub fn generate_invoice<'a>(build_path: &Path, settings_path: &Path, customer_path: &Path, invoice_path: &Path, target_path: &'a Path) -> Result<&'a Path, Box<dyn Error + Sync + Send + 'static>> {
    if !build_path.exists() && build_path.parent().unwrap().exists() {
        info!("Create build directory in {}", build_path.to_string_lossy());
        if let Err(error) = fs::create_dir(build_path) {
            error!(
                    "Unable create root directory in {}",
                    build_path.to_string_lossy()
                );
            return Err(Box::try_from(GeneratorError::UnableToCreateOutputDirectory(
                build_path.to_string_lossy().to_string(),
                error,
            ))
                .unwrap());
        }
    }

    let target_folder_path = target_path.parent().unwrap();

    if !target_folder_path.exists() && target_folder_path.parent().unwrap().exists() {
        info!("Create root directory in {}", target_folder_path.to_string_lossy());
        if let Err(error) = fs::create_dir(target_folder_path) {
            error!(
                    "Unable create target directory in {}",
                    target_folder_path.to_string_lossy()
                );
            return Err(Box::try_from(GeneratorError::UnableToCreateOutputDirectory(
                target_folder_path.to_string_lossy().to_string(),
                error,
            ))
                .unwrap());
        }
    }

    let mut main_file_type_name = target_path.to_owned();
    main_file_type_name.set_extension("typ");

    let default_template_path = build_path.to_owned().join("default_invoice_template.typ");
    let main_template_path = build_path.to_owned().join(main_file_type_name.file_name().unwrap());

    fs::write(&default_template_path, generate_default_template())?;
    fs::write(&main_template_path, generate_main_template(settings_path, customer_path, invoice_path, &default_template_path))?;


    Command::new("typst").arg("compile").arg("--root").arg("/").arg(main_template_path).arg(target_path).spawn()?;

    Ok(target_path)
}