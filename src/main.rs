use cli::parse_cli_args;

mod cli;

fn main() {
    let args = parse_cli_args(None);

    if args.get_flag("debug") {
        println!("Debug mode enabled");
    }
}
