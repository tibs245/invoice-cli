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
    /// Sets a custom config file
    #[arg(short, long, value_name = "INVOICE_ROOT_FOLDER")]
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
    Invoice {},
    /// Manage Customer
    Customer {},
    /// Show statistique
    Stats {},
}

fn main() {
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap();
    let invoice_path = cli.invoice_path.as_deref().unwrap_or(current_dir.as_path());

    match cli.debug {
        0 => env_logger::Builder::new().filter(None, LevelFilter::Warn).init(),
        1 => env_logger::Builder::new().filter(None, LevelFilter::Info).init(),
        2 => env_logger::Builder::new().filter(None, LevelFilter::Debug).init(),
        _ => env_logger::Builder::new().filter(None, LevelFilter::Trace).init(),
    }

    match &cli.command {
        Some(Commands::Init) => initiate_invoice_directory(invoice_path)
            .expect("Unable initiate this directory"),
        Some(Commands::Invoice {}) => {}
        Some(Commands::Customer {}) => {}
        Some(Commands::Stats {}) => {}
        None => {}
    };
}
