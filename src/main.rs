use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

pub mod first;

#[derive(Subcommand, Debug)]
enum Days {
    First { path: PathBuf },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Days,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Days::First { path } => first::run(path),
    }
}
