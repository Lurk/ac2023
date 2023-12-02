use clap::ValueEnum;
use std::{path::PathBuf, usize};

use crate::utils::get_non_empty_lines;

#[derive(ValueEnum, Debug, Clone)]
pub enum Type {
    Fit,
    Max,
}

#[derive(Debug)]
struct Round {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((head, tail)) = value.split_once(": ") {
            if let Some((_, id)) = head.split_once(' ') {
                return Ok(Self {
                    id: id.parse().unwrap(),
                    rounds: tail
                        .split("; ")
                        .map(|x| Round::try_from(x).unwrap())
                        .collect(),
                });
            }
        }
        Err("Invalid game")
    }
}

impl Round {
    pub fn new(red: usize, blue: usize, green: usize) -> Self {
        Self { red, blue, green }
    }

    fn does_fit_into(&self, other: &Round) -> bool {
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green
    }

    fn update_if_gt(&mut self, other: &Round) {
        if self.red < other.red {
            self.red = other.red;
        }
        if self.blue < other.blue {
            self.blue = other.blue;
        }
        if self.green < other.green {
            self.green = other.green;
        }
    }
}

impl TryFrom<&str> for Round {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut round = Round::new(0, 0, 0);
        for (number, color) in value.split(", ").map(|x| x.split_once(' ').unwrap()) {
            match color {
                "red" => round.red = number.parse().unwrap(),
                "blue" => round.blue = number.parse().unwrap(),
                "green" => round.green = number.parse().unwrap(),
                _ => return Err("Invalid key"),
            }
        }
        Ok(round)
    }
}

pub fn run(path: PathBuf, typ: &Type) {
    let requirement = Round::new(12, 14, 13);
    let mut total: usize = 0;
    for line in get_non_empty_lines(path) {
        let game = Game::try_from(line.as_str()).unwrap();
        match typ {
            Type::Fit => {
                total += if game.rounds.iter().all(|x| x.does_fit_into(&requirement)) {
                    game.id
                } else {
                    0
                };
            }
            Type::Max => {
                let mut max_round = Round::new(0, 0, 0);
                for round in game.rounds {
                    max_round.update_if_gt(&round);
                }

                total += max_round.red * max_round.blue * max_round.green;
            }
        }
    }
    println!("{}", total);
}
