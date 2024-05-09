use clap::{Parser, Subcommand};

pub mod transaction;

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gettxinfo { tx: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gettxinfo { tx } => {
            transaction::get_tx_info(tx).await.unwrap()
        }
    }
}
