use std::collections::HashSet;

use crate::{utils::get_non_empty_lines, Part, Runner};

struct Ticket {
    id: usize,
    wining_numbers: HashSet<usize>,
    numbers: Vec<usize>,
}

impl TryFrom<&str> for Ticket {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((head, tail)) = value.split_once(": ") {
            if let Some((_, id)) = head.split_once(' ') {
                if let Some((wining_numbers, numbers)) = tail.split_once(" | ") {
                    return Ok(Self {
                        id: id.trim().parse().unwrap(),
                        wining_numbers: wining_numbers
                            .split(' ')
                            .filter(|x| !x.is_empty())
                            .map(|x| x.parse().unwrap())
                            .collect(),
                        numbers: numbers
                            .split(' ')
                            .filter(|x| !x.is_empty())
                            .map(|x| x.parse().unwrap())
                            .collect(),
                    });
                }
            }
        }
        Err("Invalid ticket")
    }
}

impl Ticket {
    pub fn get_ticket_worth(&self) -> usize {
        let mut worth = 0;
        for number in &self.numbers {
            if self.wining_numbers.contains(number) {
                if worth == 0 {
                    worth = 1;
                } else {
                    worth *= 2;
                }
            }
        }
        worth
    }
}

pub fn run(runner: &Runner) {
    let mut total: u64 = 0;
    let mut tickets: Vec<Ticket> = vec![];
    for line in get_non_empty_lines(&runner.path) {
        tickets.push(Ticket::try_from(line.trim()).unwrap());
    }

    match runner.part {
        Part::One => {
            for ticket in tickets {
                total += ticket.get_ticket_worth() as u64;
            }
        }
        Part::Two => todo!(),
    }
    println!("{}", total);
}
