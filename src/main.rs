use std::path::PathBuf;

use clap::{command, Parser, Subcommand, ValueEnum};

mod fifth;
mod first;
mod fourth;
mod second;
mod sixth;
mod third;
mod utils;

#[derive(ValueEnum, Debug, Clone)]
pub enum Part {
    One,
    Two,
}

#[derive(Parser, Debug)]
pub struct Runner {
    #[clap(value_enum, value_parser)]
    pub path: PathBuf,
    pub part: Part,
}

#[derive(Subcommand, Debug)]
enum Days {
    First(Runner),
    Second(Runner),
    Third(Runner),
    Fourth(Runner),
    Fifth(Runner),
    Sixth(Runner),
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
        Days::First(runner) => first::run(runner.path),
        Days::Second(runner) => second::run(&runner),
        Days::Third(runner) => third::run(&runner),
        Days::Fourth(runner) => fourth::run(&runner),
        Days::Fifth(runner) => fifth::run(&runner),
        Days::Sixth(runner) => sixth::run(&runner),
    }
}
