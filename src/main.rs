use clap::{Args, Parser, Subcommand};
use tokio_compat_02::FutureExt;

use gotron;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a single character by its ID
    Character(GetById),
    /// Get all characters
    Characters(GetAll),
    /// Get a location character by its ID
    Location(GetById),
    /// Get all locations
    Locations(GetAll),
    /// Get a single episode by its ID
    Episode(GetById),
    /// Get all episodes
    Episodes(GetAll),
    /// Start proxy server
    Gogotron,
}

#[derive(Args)]
struct GetById {
    id: i64,

    #[arg(short, long, value_enum)]
    output: Output,
}

#[derive(Args)]
struct GetAll {
    #[arg(short, long, value_enum)]
    output: Output,
 }

 #[derive(clap::ValueEnum, Clone)]
enum Output {
   Json,
   Pretty,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Character(get_by_id) => {
            gotron::cli::print_entity(&gotron::cli::get_character(get_by_id.id).compat().await.unwrap(), if let Output::Pretty = get_by_id.output { true } else { false });
        }
        Commands::Characters(get_all) => {
            gotron::cli::print_entities(&gotron::cli::get_all_characters().compat().await.unwrap(), if let Output::Pretty = get_all.output { true } else { false });
        }
        Commands::Location(get_by_id) => {
            gotron::cli::print_entity(&gotron::cli::get_location(get_by_id.id).compat().await.unwrap(), if let Output::Pretty = get_by_id.output { true } else { false });
        }
        Commands::Locations(get_all) => {
            gotron::cli::print_entities(&gotron::cli::get_all_locations().compat().await.unwrap(), if let Output::Pretty = get_all.output { true } else { false });
        }
        Commands::Episode(get_by_id) => {
            gotron::cli::print_entity(&gotron::cli::get_episode(get_by_id.id).compat().await.unwrap(), if let Output::Pretty = get_by_id.output { true } else { false });
        }
        Commands::Episodes(get_all) => {
            gotron::cli::print_entities(&gotron::cli::get_all_episodes().compat().await.unwrap(), if let Output::Pretty = get_all.output { true } else { false });
        }
        Commands::Gogotron => {
            gotron::proxy::start_proxy_server().await;
        }
    }
}
