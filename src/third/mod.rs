use std::collections::HashSet;

use clap::ValueEnum;

use crate::{
    utils::{direction::Direction, get_non_empty_lines},
    Part, Runner,
};

#[derive(ValueEnum, Debug, Clone)]
pub enum Type {
    First,
    Second,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

fn get_part_number(map: &str, index: usize, line_length: usize) -> Option<(usize, String)> {
    let mut res = String::new();
    let mut key = String::new();
    let mut start = index;
    if index % line_length != 0 && map.chars().nth(index).unwrap().is_numeric() {
        for (index, char) in map.chars().rev().enumerate().skip(map.len() - index) {
            if index % line_length != 0 && char.is_numeric() {
                start = map.len() - index - 1;
            } else {
                break;
            }
        }
    }

    for (index, char) in map.chars().enumerate().skip(start) {
        if char.is_numeric() {
            key.push_str(index.to_string().as_str());
            res.push(char);
        } else {
            break;
        }

        if index % line_length == line_length - 1 {
            break;
        }
    }

    if res.is_empty() {
        None
    } else {
        Some((res.parse().unwrap(), key))
    }
}

fn get_sum_of_parts(map: &str, line_length: usize) -> usize {
    let mut total = 0;
    let mut visited: HashSet<String> = HashSet::new();
    for (char_index, char) in map.chars().enumerate() {
        match char {
            '$' | '#' | '+' | '/' | '@' | '-' | '=' | '%' | '*' | '&' => {
                for direction in DIRECTIONS.iter() {
                    if let Some(index) = direction.get_index(map.len(), line_length, char_index) {
                        if let Some((part_number, key)) = get_part_number(map, index, line_length) {
                            if !visited.contains(&key) {
                                total += part_number;
                                visited.insert(key);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    total
}

fn get_sum_of_gear_ratios(map: &str, line_length: usize) -> usize {
    let mut total = 0;
    let mut visited: HashSet<String> = HashSet::new();
    for (char_index, char) in map.chars().enumerate() {
        if char == '*' {
            let mut gears = vec![];
            for direction in DIRECTIONS.iter() {
                if let Some(index) = direction.get_index(map.len(), line_length, char_index) {
                    if let Some((part_number, key)) = get_part_number(map, index, line_length) {
                        if !visited.contains(&key) {
                            visited.insert(key);
                            gears.push(part_number);
                        }
                    }
                }
            }
            if gears.len() == 2 {
                total += gears[0] * gears[1];
            }
        }
    }
    total
}

pub fn run(runner: &Runner) {
    let mut line_length: usize = 0;
    let mut map: String = String::new();
    for line in get_non_empty_lines(&runner.path) {
        let line = line.trim();
        line_length = line.len();
        map.push_str(line);
    }
    match runner.part {
        Part::One => {
            println!("{}", get_sum_of_parts(&map, line_length));
        }
        Part::Two => {
            println!("{}", get_sum_of_gear_ratios(&map, line_length));
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_part_number() {
        // .12
        // 345
        // 67.
        let map = ".1234567.";
        let line_length = 3;

        assert_eq!(super::get_part_number(map, 0, line_length), None);
        assert_eq!(
            super::get_part_number(map, 1, line_length),
            Some((12, "12".to_string()))
        );
        assert_eq!(
            super::get_part_number(map, 2, line_length),
            Some((12, "12".to_string()))
        );
        assert_eq!(
            super::get_part_number(map, 3, line_length),
            Some((345, "345".to_string()))
        );
        assert_eq!(
            super::get_part_number(map, 4, line_length),
            Some((345, "345".to_string()))
        );
        assert_eq!(
            super::get_part_number(map, 5, line_length),
            Some((345, "345".to_string()))
        );
        assert_eq!(
            super::get_part_number(map, 6, line_length),
            Some((67, "67".to_string()))
        );
        assert_eq!(
            super::get_part_number(map, 7, line_length),
            Some((67, "67".to_string()))
        );
        assert_eq!(super::get_part_number(map, 8, line_length), None);
    }

    #[test]
    fn get_sum_of_parts() {
        // 467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..
        let map = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";
        let line_length = 10;

        assert_eq!(
            super::get_sum_of_parts(map, line_length),
            467 + 35 + 633 + 617 + 592 + 755 + 664 + 598
        );
    }
}
