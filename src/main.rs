use std::env;
use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::cli::cli_error::CliError;
use crate::cli::create_customer::create_customer;
use crate::cli::create_invoice::create_invoice;
use crate::cli::day_stats::day_stats;
use crate::cli::delete_customer::delete_customer;
use crate::cli::delete_invoice::cancel_invoice;
use crate::cli::edit_customer::edit_customer;
use crate::cli::edit_settings::edit_settings;
use crate::cli::generate_all_invoice::generate_all_invoice;
use crate::cli::generate_invoice::generate_invoice;
use crate::cli::get_customer::get_customer;
use crate::cli::get_invoice::get_invoice;
use crate::cli::get_settings::get_settings;
use crate::cli::init::initiate_invoice_directory;
use crate::cli::list_customers::list_customers;
use crate::cli::list_invoices::list_invoices;
use crate::cli::month_stats::month_stats;
use crate::cli::year_stats::year_stats;
use crate::file_manager::context_parameters::ContextParameters;

mod cli;
mod entities;
mod file_manager;
mod generator;
mod invoice_manager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom default root path
    #[arg(long, value_name = "FILE")]
    root_path: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(long, value_name = "FILE")]
    config_file_path: Option<PathBuf>,

    /// Sets a custom customer file
    #[arg(long, value_name = "FILE")]
    customer_file_path: Option<PathBuf>,

    /// Sets a custom invoice file
    #[arg(long, value_name = "FILE")]
    invoice_path: Option<PathBuf>,

    /// Sets a custom invoice file
    #[arg(long, value_name = "FILE")]
    build_path: Option<PathBuf>,

    /// Sets a custom invoice file
    #[arg(long, value_name = "FILE")]
    target_path: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Init invoices path
    Init,
    /// Manage Invoice
    Invoice {
        #[command(subcommand)]
        action: Option<CrudAction>,
    },
    /// Manage Customer
    Customer {
        #[command(subcommand)]
        action: Option<CrudAction>,
    },
    /// Show stats
    Stats {
        #[command(subcommand)]
        action: Option<StatsAction>
    },
    /// Manage settings and enterprise informations
    Settings {
        #[command(subcommand)]
        action: Option<CrudAction>,
    },
    /// Generate PDF for a invoice
    Generate {
        invoice: Option<String>
    },
    /// Generate PDF for all invoices saved
    GenerateAll,
}

#[derive(Subcommand)]
enum CrudAction {
    Create,
    List,
    Get {
        element: Option<String>
    },
    Edit {
        element: Option<String>
    },
    Delete {
        element: Option<String>
    },
}


#[derive(Subcommand)]
enum StatsAction {
    Day {
        day: Option<u32>,
        month: Option<u32>,
        year: Option<i32>,
    },
    Month {
        month: Option<u32>,
        year: Option<i32>,
    },
    Year {
        year: Option<i32>
    },
}

fn main() {
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap();
    let invoice_manager_path = cli.root_path.as_deref().unwrap_or(current_dir.as_path());

    match cli.debug {
        0 => env_logger::Builder::new().filter(None, LevelFilter::Warn).init(),
        1 => env_logger::Builder::new().filter(None, LevelFilter::Info).init(),
        2 => env_logger::Builder::new().filter(None, LevelFilter::Debug).init(),
        _ => env_logger::Builder::new().filter(None, LevelFilter::Trace).init(),
    }

    let parameters = ContextParameters {
        invoice_manager_path,
        invoice_path: cli.invoice_path.as_deref(),
        config_file_path: cli.config_file_path.as_deref(),
        customer_file_path: cli.customer_file_path.as_deref(),
        build_path: cli.build_path.as_deref(),
        target_path: cli.target_path.as_deref(),
    };

    let result: Result<(), Box<dyn Error + Sync + Send + 'static>> = match &cli.command {
        Some(Commands::Init) => initiate_invoice_directory(
            parameters
        ),
        Some(Commands::Invoice { action }) => match action {
            Some(CrudAction::List) => list_invoices(parameters),
            Some(CrudAction::Get { element }) => get_invoice(parameters, element),
            Some(CrudAction::Create) => create_invoice(parameters),
            Some(CrudAction::Edit { element: _element }) => {
                Err(Box::new(CliError::CommandNotExists("You can't edit a invoice. You can only cancel the old invoice and create another".to_string())))
            }
            Some(CrudAction::Delete { element }) => cancel_invoice(parameters, element),
            None => {
                Err(Box::new(CliError::CommandNotExists("You can get, create or delete invoice".to_string())))
            }
        },
        Some(Commands::Customer { action }) => match action {
            Some(CrudAction::List) => list_customers(parameters),
            Some(CrudAction::Get { element }) => get_customer(parameters, element),
            Some(CrudAction::Create) => create_customer(parameters),
            Some(CrudAction::Edit { element }) => { edit_customer(parameters, element) }
            Some(CrudAction::Delete { element }) => { delete_customer(parameters, element) }
            None => { Err(Box::new(CliError::NotImplementedYet())) }
        },
        Some(Commands::Stats { action }) => {
            match action {
                Some(StatsAction::Day { day, month, year }) => { day_stats(parameters, day, month, year) }
                Some(StatsAction::Month { month, year }) => { month_stats(parameters, month, year) }
                Some(StatsAction::Year { year }) => { year_stats(parameters, year) }
                None => { Err(Box::new(CliError::NotImplementedYet())) }
            }
        }
        Some(Commands::Settings { action }) => match action {
            Some(CrudAction::List) => todo!("Not implemented"),
            Some(CrudAction::Get { element: _element }) => get_settings(parameters),
            Some(CrudAction::Create) => {
                println!("This command init a new folder invoice. Don't use it on a already initiated folder");
                initiate_invoice_directory(parameters)
            }
            Some(CrudAction::Edit { element: _element }) => { edit_settings(parameters) }
            Some(CrudAction::Delete { element: _element }) => Err(Box::new(CliError::CommandNotExists("Not implemented, If you want delete the folder you can delete all files manually".to_string()))),
            None => { Err(Box::new(CliError::NotImplementedYet())) }
        },
        Some(Commands::Generate { invoice }) => generate_invoice(parameters, invoice),
        Some(Commands::GenerateAll) => generate_all_invoice(parameters),
        None => Err(Box::new(CliError::CommandNotExists("The option is not correct. Try to get help".to_string())))
    };

    result.unwrap_or_else(|error| println!("Error : {}", error));
}
