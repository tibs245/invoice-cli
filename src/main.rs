use std::env;
use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::cli::cli_error::CliError;
use crate::cli::context_parameters::ContextParameters;
use crate::cli::create_customer::create_customer;
use crate::cli::create_invoice::create_invoice;
use crate::cli::delete_customer::delete_customer;
use crate::cli::delete_invoice::delete_invoice;
use crate::cli::get_customer::get_customer;
use crate::cli::get_invoice::get_invoice;
use crate::cli::init::initiate_invoice_directory;
use crate::cli::list_customers::list_customers;
use crate::cli::list_invoices::list_invoices;

mod cli;
mod entities;
mod file_manager;

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
    Stats {},
}

#[derive(Subcommand)]
enum CrudAction {
    Create,
    List,
    Get {
        element: Option<String>
    },
    Edit,
    Delete {
        element: Option<String>
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
    };

    let result: Result<(), Box<dyn Error + Sync + Send + 'static>> = match &cli.command {
        Some(Commands::Init) => initiate_invoice_directory(
            parameters
        ),
        Some(Commands::Invoice { action }) => match action {
            Some(CrudAction::List) => list_invoices(parameters),
            Some(CrudAction::Get { element }) => get_invoice(parameters, element),
            Some(CrudAction::Create) => create_invoice(parameters),
            Some(CrudAction::Edit) => {
                Err(Box::new(CliError::CommandNotExists("You can't edit a invoice. You can only delete the old invoice and create another".to_string())))
            }
            Some(CrudAction::Delete { element }) => delete_invoice(parameters),
            None => {
                Err(Box::new(CliError::CommandNotExists("You can get, create or delete invoice".to_string())))
            }
        },
        Some(Commands::Customer { action }) => match action {
            Some(CrudAction::List) => list_customers(parameters),
            Some(CrudAction::Get { element }) => get_customer(parameters, element),
            Some(CrudAction::Create) => create_customer(parameters),
            Some(CrudAction::Edit) => { Err(Box::new(CliError::NotImplementedYet())) }
            Some(CrudAction::Delete { element }) => { delete_customer(parameters, element) }
            None => { Err(Box::new(CliError::NotImplementedYet())) }
        },
        Some(Commands::Stats {}) => { Err(Box::new(CliError::NotImplementedYet())) }
        None => Err(Box::new(CliError::CommandNotExists("The option is not correct. Try to get help".to_string())))
    };

    result.unwrap_or_else(|error| println!("Error : {}", error));
}
