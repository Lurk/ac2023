use std::{usize, vec};

use crate::{
    utils::{direction::Direction, get_non_empty_lines, map::Map},
    Part, Runner,
};

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount_of_steps: usize,
}

impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        let chunks: Vec<&str> = val.trim().split(' ').collect();
        Instruction {
            direction: match chunks[0] {
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                "U" => Direction::North,
                _ => panic!("Invalid direction"),
            },
            amount_of_steps: chunks[1].parse().unwrap(),
        }
    }
}

impl From<String> for Instruction {
    fn from(val: String) -> Self {
        Instruction::from(val.as_str())
    }
}

fn build_map(instructions: impl Iterator<Item = Instruction>) -> Map<char> {
    let mut map = Map {
        tiles: vec!['0'],
        line_length: 1,
    };

    let mut index = 0;
    for instruction in instructions {
        let distance_to_border = map.distance_to_border(index, &instruction.direction);
        if distance_to_border < instruction.amount_of_steps {
            let (_, y) = map.to_xy(index);
            let amount = instruction.amount_of_steps - distance_to_border;
            map.extend(&instruction.direction, amount, '.');
            if instruction.direction == Direction::West {
                index += amount * (y + 1);
            } else if instruction.direction == Direction::North {
                index += amount * map.line_length;
            } else if instruction.direction == Direction::East {
                index += amount * y;
            }
        }
        for _ in 0..instruction.amount_of_steps {
            index = map
                .move_from(index, &instruction.direction)
                .unwrap_or_else(|| {
                    panic!(
                        "should able to move from {} to {}",
                        index, instruction.direction
                    )
                });
            map.tiles[index] = instruction.direction.clone().into();
        }
    }
    map
}

fn fill(map: &mut Map<char>) {
    for index in 0..map.tiles.len() {
        let d = match map.tiles[index] {
            'n' => vec![Direction::East],
            'e' => vec![Direction::South],
            's' => vec![Direction::West],
            'w' => vec![Direction::North],
            'i' => vec![
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North,
            ],
            _ => vec![],
        };
        for direction in d {
            if let Some(i) = map.move_from(index, &direction) {
                if map.tiles[i] == '.' {
                    map.tiles[i] = 'i';
                }
            }
        }
    }
}

fn two() -> usize {
    todo!()
}

fn one(input: impl Iterator<Item = String>) -> usize {
    let mut map = build_map(input.map(Instruction::from));
    fill(&mut map);
    map.tiles.iter().filter(|x| **x != '.').count()
}

pub fn run(runner: &Runner) {
    let result = match runner.part {
        Part::One => one(get_non_empty_lines(&runner.path)),
        Part::Two => two(),
    };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

    #[test]
    fn bm() {
        let instructions = TEST_INPUT.trim().lines().map(Instruction::from);

        let map = build_map(instructions);
        assert_eq!(map.tiles.len(), 70);
        assert_eq!(map.line_length, 7);
    }

    #[test]
    fn test_one() {
        assert_eq!(one(TEST_INPUT.trim().lines().map(|s| s.to_string())), 62);
    }
}
