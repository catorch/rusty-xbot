use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn parse_cli_args(argv: Option<Vec<String>>) -> ArgMatches {
    Command::new("RustXBot")
        .arg(
            Arg::new("debug_mode")
                .long("debug")
                .help("Enables verbose debug logging for troubleshooting")
                .action(ArgAction::SetTrue),
        )
        .get_matches_from(argv.unwrap_or_else(|| std::env::args().collect()))
}
