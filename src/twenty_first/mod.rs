use std::{
    collections::{HashMap, HashSet, VecDeque},
    i64,
};

use crate::{
    utils::{direction::Direction, get_non_empty_lines, map::Map},
    Part, Runner,
};

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

fn parse_input(input: impl Iterator<Item = String>) -> Map<char> {
    let mut map = Map {
        tiles: vec![],
        line_length: 0,
    };
    let rows = input
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    map.replace_rows(rows);
    map
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
    multiplier_y: usize,
    multiplier_x: usize,
}

impl Position {
    fn move_to(&self, direction: &Direction, max_x: usize, max_y: usize) -> Position {
        match direction {
            Direction::North => {
                if self.y == 0 {
                    Position {
                        x: self.x,
                        y: max_y - 1,
                        multiplier_y: self.multiplier_y + 1,
                        multiplier_x: self.multiplier_x,
                    }
                } else {
                    Position {
                        x: self.x,
                        y: self.y - 1,
                        multiplier_y: self.multiplier_y,
                        multiplier_x: self.multiplier_x,
                    }
                }
            }
            Direction::South => {
                if self.y == max_y - 1 {
                    Position {
                        x: self.x,
                        y: 0,
                        multiplier_y: self.multiplier_y - 1,
                        multiplier_x: self.multiplier_x,
                    }
                } else {
                    Position {
                        x: self.x,
                        y: self.y + 1,
                        multiplier_y: self.multiplier_y,
                        multiplier_x: self.multiplier_x,
                    }
                }
            }
            Direction::West => {
                if self.x == 0 {
                    Position {
                        x: max_x - 1,
                        y: self.y,
                        multiplier_y: self.multiplier_y,
                        multiplier_x: self.multiplier_x + 1,
                    }
                } else {
                    Position {
                        x: self.x - 1,
                        y: self.y,
                        multiplier_y: self.multiplier_y,
                        multiplier_x: self.multiplier_x,
                    }
                }
            }
            Direction::East => {
                if self.x == max_x - 1 {
                    Position {
                        x: 0,
                        y: self.y,
                        multiplier_y: self.multiplier_y,
                        multiplier_x: self.multiplier_x - 1,
                    }
                } else {
                    Position {
                        x: self.x + 1,
                        y: self.y,
                        multiplier_y: self.multiplier_y,
                        multiplier_x: self.multiplier_x,
                    }
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    fn to_index(&self, max_x: usize, max_y: usize) -> usize {
        let x = max_x * self.multiplier_x;
        self.y * x + max_y * self.multiplier_y * x + self.x + x
    }
}

fn go2(map: &Map<char>, amount_of_steps: usize) -> usize {
    let mut deq: VecDeque<Position> = VecDeque::new();
    map.tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| **tile == 'S')
        .for_each(|(i, _)| {
            let (x, y) = map.to_xy(i);
            deq.push_back(Position {
                x,
                y,
                multiplier_y: 100000,
                multiplier_x: 100000,
            });
        });

    let mut derivative: i64 = 0;
    let mut second_derivative: i64 = 0;
    let mut prev_length = 0;
    let mut current_length = deq.len();
    let mut derivatives: HashMap<i64, Vec<usize>> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    for step in 0..amount_of_steps {
        visited.clear();
        for _ in 0..current_length {
            let position = deq.pop_front().expect("No position");
            for d in DIRECTIONS {
                let new_position = position.move_to(&d, map.line_length, map.get_rows_count());
                let index = map.to_index(new_position.x, new_position.y);
                if map.tiles[index] != '#' && !visited.contains(&new_position) {
                    deq.push_back(new_position.clone());
                    visited.insert(new_position);
                }
            }
        }

        let value = current_length - prev_length;
        derivative = value as i64 - derivative;
        second_derivative = derivative as i64 - second_derivative;

        derivatives
            .entry(second_derivative)
            .or_insert(vec![])
            .push(step);

        prev_length = current_length;
        current_length = deq.len();
        second_derivative = derivative as i64;
        derivative = value as i64;
    }
    derivatives
        .iter()
        .filter(|(_, v)| v.len() > 3)
        .for_each(|(k, v)| {
            println!("{:?}: {:?}", k, v);
        });
    current_length
}

fn go(map: &mut Map<char>, amount_of_steps: usize) -> usize {
    for _ in 0..amount_of_steps {
        let start_positions: Vec<usize> = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == 'S')
            .map(|(i, _)| i)
            .collect();

        for start_position in start_positions {
            for d in DIRECTIONS {
                if let Some(new_position) = map.move_from(start_position, &d) {
                    if map.tiles[new_position] != '#' {
                        map.tiles[new_position] = 'S';
                        map.tiles[start_position] = '.';
                    }
                }
            }
        }
    }
    map.tiles.iter().filter(|tile| **tile == 'S').count()
}

fn one(map: &mut Map<char>) -> usize {
    go2(map, 64)
}

fn two(map: &mut Map<char>) -> usize {
    go2(map, 1000)
}

pub fn run(runner: &Runner) {
    let mut map = parse_input(get_non_empty_lines(&runner.path));
    let result = match runner.part {
        Part::One => one(&mut map),
        Part::Two => two(&mut map),
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

    #[test]
    fn test_go() {
        let mut map = parse_input(INPUT.trim().lines().map(|s| s.to_string()));
        assert_eq!(go(&mut map, 6), 16);
    }

    #[test]
    fn test_two() {
        let map = parse_input(INPUT.trim().lines().map(|s| s.to_string()));
        println!("{}", map.tiles.iter().filter(|tile| **tile != '#').count());
        let now = std::time::Instant::now();
        assert_eq!(go2(&map, 6), 16);
        println!("6: {:?}", now.elapsed());
        assert_eq!(go2(&map, 10), 50);
        println!("10: {:?}", now.elapsed());
        assert_eq!(go2(&map, 50), 1594);
        println!("50: {:?}", now.elapsed());
        assert_eq!(go2(&map, 100), 6536);
        println!("100: {:?}", now.elapsed());
        assert_eq!(go2(&map, 500), 167004);
        println!("500: {:?}", now.elapsed());
        assert_eq!(go2(&map, 1000), 668697);
        println!("1000: {:?}", now.elapsed());
        assert_eq!(go2(&map, 5000), 16733044);
        println!("5000: {:?}", now.elapsed());
    }
}
