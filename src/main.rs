mod commands;
mod show;
use clap::Parser;

use std::process;

fn main() {
    let cli = commands::Cli::parse();

    match &cli.command {
        commands::Commands::Show(args) => {
            if let Err(err) = show::parse_and_display_csv(&cli.common, args) {
                eprintln!("Error: {}", err);
                process::exit(1);
            };
        }
    };
}
