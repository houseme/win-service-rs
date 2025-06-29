mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use win_service_rs::ServiceManager;

#[derive(Parser)]
#[clap(
    name = "rustfs-manager",
    about = "Service manager for rustfs.exe using win-service-rs"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the rustfs service
    Start,
    /// Stop the rustfs service
    Stop,
    /// Query the status of the rustfs service
    Status,
    /// Install the rustfs service
    Install,
    /// Uninstall the rustfs service
    Uninstall,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(filter)
        .init();

    let cli = Cli::parse();
    let config = config::load_config()?;
    let service_config = config.service.to_win_service_config();
    let mut manager = ServiceManager::new(&config.service.name)?;

    match cli.command {
        Commands::Start => {
            manager.start()?;
            tracing::info!(service = %config.service.name, "Service started");
            println!("Service {} started", config.service.name);
        }
        Commands::Stop => {
            manager.stop()?;
            tracing::info!(service = %config.service.name, "Service stopped");
            println!("Service {} stopped", config.service.name);
        }
        Commands::Status => {
            let status = manager.status()?;
            tracing::info!(service = %config.service.name, ?status, "Service status queried");
            println!("Service {}: {:?}", config.service.name, status);
        }
        Commands::Install => {
            manager.install(&service_config)?;
            tracing::info!(service = %config.service.name, "Service installed");
            println!("Service {} installed", config.service.name);
        }
        Commands::Uninstall => {
            manager.uninstall()?;
            tracing::info!(service = %config.service.name, "Service uninstalled");
            println!("Service {} uninstalled", config.service.name);
        }
    }
    Ok(())
}
