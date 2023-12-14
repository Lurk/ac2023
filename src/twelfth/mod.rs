use std::{
    fmt::{Display, Formatter},
    usize,
};

use crate::{
    utils::{combinations, get_non_empty_lines},
    Part, Runner,
};

struct Line {
    springs: Vec<char>,
    numbers: Vec<usize>,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.springs
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(""),
            self.numbers
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Line {
    fn get_amount_if_combinations(&self) -> usize {
        let size: usize = self.numbers.iter().sum::<usize>() + self.numbers.len() - 1;
        let line_size: usize = self.springs.len();

        let n = combinations(
            (0..self.numbers.len() + line_size - size)
                .collect::<Vec<usize>>()
                .as_slice(),
            self.numbers.len(),
        )
        .iter()
        .fold(0, |acc, combination| {
            let mut index = 0;
            let mut prev: usize = 0;
            for i in 0..combination.len() {
                let amount_of_dots = combination[i] - prev;
                let amount_of_hashes = self.numbers[i];
                if self.springs[index..index + amount_of_dots]
                    .iter()
                    .any(|c| *c == '#')
                {
                    return acc;
                }

                if self.springs[index + amount_of_dots..index + amount_of_dots + amount_of_hashes]
                    .iter()
                    .any(|c| *c == '.')
                {
                    return acc;
                }
                prev = combination[i];
                index += amount_of_dots + amount_of_hashes;
            }
            if self.springs[index..].iter().any(|c| *c == '#') {
                return acc;
            }
            acc + 1
        });
        // println!("line: {}\t\thas {} combinations ", self, n);
        n
    }
}

fn parse_line(line: &str) -> Line {
    let (head, tail) = line.split_once(' ').unwrap();

    let numbers: Vec<usize> = tail
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let springs = head.trim_matches('.').chars().fold(vec![], |mut acc, c| {
        // remove duplicate dots
        if c == '.' && acc.last() == Some(&'.') {
            acc
        } else {
            acc.push(c);
            acc
        }
    });

    Line { springs, numbers }
}

fn one(lines: impl Iterator<Item = Line>) -> usize {
    lines.fold(0, |acc, line| acc + line.get_amount_if_combinations())
}

fn two() -> usize {
    0
}

pub fn run(runner: &Runner) {
    let lines = get_non_empty_lines(&runner.path).map(|line| parse_line(line.trim()));

    let result = match runner.part {
        Part::One => one(lines),
        Part::Two => two(),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("???.### 1,1,3").get_amount_if_combinations(), 1);
        assert_eq!(
            parse_line("..??..??...?##.. 1,1,3").get_amount_if_combinations(),
            4
        );
        assert_eq!(
            parse_line("?#?#?#?#?#?#?#? 1,3,1,6").get_amount_if_combinations(),
            1
        );
        assert_eq!(
            parse_line("????.######..#####. 1,6,5").get_amount_if_combinations(),
            4
        );

        assert_eq!(
            parse_line("?###???????? 3,2,1").get_amount_if_combinations(),
            10
        );

        assert_eq!(
            parse_line("????.#...#... 4,1,1").get_amount_if_combinations(),
            1
        );

        assert_eq!(parse_line("#.#?. 1,1").get_amount_if_combinations(), 1);
        assert_eq!(parse_line("??#???##?? 1,3").get_amount_if_combinations(), 2);
        assert_eq!(parse_line("#??# 2").get_amount_if_combinations(), 0);
    }

    #[test]
    fn test_one() {
        let lines = vec![
            parse_line("???.### 1,1,3"),
            parse_line(".??..??...?##. 1,1,3"),
            parse_line("?#?#?#?#?#?#?#? 1,3,1,6"),
            parse_line("????.######..#####. 1,6,5"),
            parse_line("?###???????? 3,2,1"),
            parse_line("????.#...#... 4,1,1"),
        ];
        assert_eq!(one(lines.into_iter()), 21);
    }
}
