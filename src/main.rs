use std::path::PathBuf;

use clap::{command, Parser, Subcommand};
use second::Type;

mod first;
mod second;

#[derive(Subcommand, Debug)]
enum Days {
    First {
        path: PathBuf,
    },
    Second {
        path: PathBuf,
        #[clap(value_enum, value_parser)]
        typ: Type,
    },
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
        Days::Second { path, typ } => second::run(path, &typ),
    }
}
