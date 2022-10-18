use clap::Command;

use gotron;

fn main() {
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
            gotron::list_characters();
        },
        Some(("locations", _)) => {
            gotron::list_locations();
        },
        Some(("episodes", _)) => {
            gotron::list_episodes();
        },
        Some(("gogotron", _)) => {
            gotron::start_proxy_server();
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
