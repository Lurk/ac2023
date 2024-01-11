use std::path::PathBuf;

use clap::{command, Parser, Subcommand, ValueEnum};

mod eighteenth;
mod eighth;
mod eleventh;
mod fifteenth;
mod fifth;
mod first;
mod fourteenth;
mod fourth;
mod nineteenth;
mod ninth;
mod second;
mod seventeenth;
mod seventh;
mod sixteenth;
mod sixth;
mod tenth;
mod third;
mod thirteenth;
mod twelfth;
mod twentieth;
mod twenty_first;
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
    Seventh(Runner),
    Eighth(Runner),
    Ninth(Runner),
    Tenth(Runner),
    Eleventh(Runner),
    Twelfth(Runner),
    Thirteenth(Runner),
    Fourteenth(Runner),
    Fifteenth(Runner),
    Sixteenth(Runner),
    Seventeenth(Runner),
    Eighteenth(Runner),
    Nineteenth(Runner),
    Twentieth(Runner),
    TwentyFirst(Runner),
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
        Days::Seventh(runner) => seventh::run(&runner),
        Days::Eighth(runner) => eighth::run(&runner),
        Days::Ninth(runner) => ninth::run(&runner),
        Days::Tenth(runner) => tenth::run(&runner),
        Days::Eleventh(runner) => eleventh::run(&runner),
        Days::Twelfth(runner) => twelfth::run(&runner),
        Days::Thirteenth(runner) => thirteenth::run(&runner),
        Days::Fourteenth(runner) => fourteenth::run(&runner),
        Days::Fifteenth(runner) => fifteenth::run(&runner),
        Days::Sixteenth(runner) => sixteenth::run(&runner),
        Days::Seventeenth(runner) => seventeenth::run(&runner),
        Days::Eighteenth(runner) => eighteenth::run(&runner),
        Days::Nineteenth(runner) => nineteenth::run(&runner),
        Days::Twentieth(runner) => twentieth::run(&runner),
        Days::TwentyFirst(runner) => twenty_first::run(&runner),
    }
}
