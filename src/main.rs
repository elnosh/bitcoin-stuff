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
    Gettxinfo { rawtx: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gettxinfo { rawtx } => {
            transaction::get_tx_info(rawtx).unwrap();
        }
    }
}
