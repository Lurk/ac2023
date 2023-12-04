use std::{collections::HashSet, usize};

use crate::{utils::get_non_empty_lines, Part, Runner};

struct Ticket {
    wining_numbers: HashSet<usize>,
    numbers: Vec<usize>,
}

impl TryFrom<&str> for Ticket {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((_, tail)) = value.split_once(": ") {
            if let Some((wining_numbers, numbers)) = tail.split_once(" | ") {
                return Ok(Self {
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

    pub fn get_matching_count(&self) -> usize {
        let mut count = 0;
        for number in &self.numbers {
            if self.wining_numbers.contains(number) {
                count += 1;
            }
        }
        count
    }
}

pub fn run(runner: &Runner) {
    let mut total: usize = 0;
    let mut tickets: Vec<Ticket> = vec![];
    for line in get_non_empty_lines(&runner.path) {
        tickets.push(Ticket::try_from(line.trim()).unwrap());
    }

    match runner.part {
        Part::One => {
            for ticket in tickets {
                total += ticket.get_ticket_worth();
            }
        }
        Part::Two => {
            total = tickets.len();
            let mut multipliers: Vec<usize> = vec![1; tickets.len()];
            for i in 0..tickets.len() {
                let count = tickets[i].get_matching_count();
                for im in i + 1..i + 1 + count {
                    multipliers[im] += multipliers[i];
                }
            }

            for i in 0..tickets.len() {
                total += tickets[i].get_matching_count() * multipliers[i];
            }
        }
    }
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticket() {
        let ticket = Ticket::try_from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        assert_eq!(ticket.wining_numbers.len(), 5);
        assert_eq!(ticket.numbers.len(), 8);
        assert_eq!(ticket.get_ticket_worth(), 8);
        assert_eq!(ticket.get_matching_count(), 4);
    }
}
