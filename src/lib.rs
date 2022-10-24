use clap::{Args, Parser, Subcommand};
use tokio_compat_02::FutureExt;
use std::error::Error;

pub mod cli;
pub mod proxy;

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
pub enum Output {
   Json,
   Pretty,
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Character(get_by_id) => {
            cli::print_entity(&cli::get_character(get_by_id.id).compat().await?, &get_by_id.output);
        }
        Commands::Characters(get_all) => {
            cli::print_entities(&cli::get_all_characters().compat().await?, &get_all.output);
        }
        Commands::Location(get_by_id) => {
            cli::print_entity(&cli::get_location(get_by_id.id).compat().await?, &get_by_id.output);
        }
        Commands::Locations(get_all) => {
            cli::print_entities(&cli::get_all_locations().compat().await?, &get_all.output);
        }
        Commands::Episode(get_by_id) => {
            cli::print_entity(&cli::get_episode(get_by_id.id).compat().await?, &get_by_id.output);
        }
        Commands::Episodes(get_all) => {
            cli::print_entities(&cli::get_all_episodes().compat().await?, &get_all.output);
        }
        Commands::Gogotron => {
            proxy::start_proxy_server().await;
        }
    };

    Ok(())
}