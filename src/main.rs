use clap::{Args, Parser, Subcommand};
use tokio_compat_02::FutureExt;

use gotron;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a single character by its ID
    Character(GetById),
    /// Get all characters
    Characters,
    /// Get a location character by its ID
    Location(GetById),
    /// Get all locations
    Locations,
    /// Get a single episode by its ID
    Episode(GetById),
    /// Get all episodes
    Episodes,
    /// Start proxy server
    Gogotron,
}

#[derive(Args)]
struct GetById {
    id: i64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Character(get_by_id) => {
            gotron::cli::show_character(get_by_id.id).compat().await;
        }
        Commands::Characters => {
            gotron::cli::list_characters().compat().await;
        }
        Commands::Location(get_by_id) => {
            gotron::cli::show_location(get_by_id.id).compat().await;
        }
        Commands::Locations => {
            gotron::cli::list_locations().compat().await;
        }
        Commands::Episode(get_by_id) => {
            gotron::cli::show_episode(get_by_id.id).compat().await;
        }
        Commands::Episodes => {
            gotron::cli::list_episodes().compat().await;
        }
        Commands::Gogotron => {
            gotron::proxy::start_proxy_server().await;
        }
    }
}
