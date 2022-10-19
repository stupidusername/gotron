use clap::Command;

use gotron;

#[tokio::main]
async fn main() {
    let matches = Command::new("GoTron")
        .subcommand_required(true)
        .subcommand(
            Command::new("characters")
                .about("Get characters")
        )
        .subcommand(
            Command::new("locations")
                .about("Get locations")
        )
        .subcommand(
            Command::new("episodes")
                .about("Get episodes")
        )
        .subcommand(
            Command::new("gogotron")
                .about("Start proxy server")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("characters", _)) => {
            gotron::list_characters().await;
        },
        Some(("locations", _)) => {
            gotron::list_locations().await;
        },
        Some(("episodes", _)) => {
            gotron::list_episodes().await;
        },
        Some(("gogotron", _)) => {
            gotron::start_proxy_server().await;
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
