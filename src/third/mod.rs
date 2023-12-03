use clap::ValueEnum;

use crate::{utils::get_non_empty_lines, Runner};

#[derive(ValueEnum, Debug, Clone)]
pub enum Type {
    First,
    Second,
}

pub fn run(runner: &Runner) {
    let mut total: u64 = 0;
    for line in get_non_empty_lines(&runner.path) {}
    println!("{}", total);
}
