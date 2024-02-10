use crate::cli::create_customer::create_customer;
use crate::cli::init::initiate_invoice_directory;
use clap::{Parser, Subcommand};
use log::LevelFilter;
use std::env;
use std::path::PathBuf;

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
    /// Show statistique
    Stats {},
}

#[derive(Subcommand)]
enum CrudAction {
    Create,
    Get,
    Edit,
    Delete,
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

    match &cli.command {
        Some(Commands::Init) => initiate_invoice_directory(
            invoice_manager_path,
            cli.invoice_path.as_deref(),
            cli.config_file_path.as_deref(),
            cli.customer_file_path.as_deref(),
        )
        .unwrap(),
        Some(Commands::Invoice { action }) => match action {
            Some(CrudAction::Get) => {}
            Some(CrudAction::Create) => {}
            Some(CrudAction::Edit) => {}
            Some(CrudAction::Delete) => {}
            None => {}
        },
        Some(Commands::Customer { action }) => match action {
            Some(CrudAction::Get) => {}
            Some(CrudAction::Create) => create_customer(
                invoice_manager_path,
                cli.invoice_path.as_deref(),
                cli.config_file_path.as_deref(),
                cli.customer_file_path.as_deref(),
            )
            .unwrap(),
            Some(CrudAction::Edit) => {}
            Some(CrudAction::Delete) => {}
            None => {}
        },
        Some(Commands::Stats {}) => {}
        None => {}
    };
}
