use std::{collections::HashSet, usize};

use crate::{
    utils::map::Map,
    utils::{direction::Direction, get_non_empty_lines},
    Part, Runner,
};

fn next(map: &Map<char>, index: usize, direction: Direction) -> Vec<(Direction, usize)> {
    if map.tiles[index] == '.' {
        return vec![(direction, index)];
    } else if map.tiles[index] == '|' {
        if direction == Direction::North || direction == Direction::South {
            return vec![(direction, index)];
        } else if direction == Direction::East || direction == Direction::West {
            return vec![(Direction::North, index), (Direction::South, index)];
        }
    } else if map.tiles[index] == '-' {
        if direction == Direction::East || direction == Direction::West {
            return vec![(direction, index)];
        } else if direction == Direction::North || direction == Direction::South {
            return vec![(Direction::East, index), (Direction::West, index)];
        }
    } else if map.tiles[index] == '/' {
        if direction == Direction::North {
            return vec![(Direction::East, index)];
        } else if direction == Direction::East {
            return vec![(Direction::North, index)];
        } else if direction == Direction::South {
            return vec![(Direction::West, index)];
        } else if direction == Direction::West {
            return vec![(Direction::South, index)];
        }
    } else if map.tiles[index] == '\\' {
        if direction == Direction::North {
            return vec![(Direction::West, index)];
        } else if direction == Direction::East {
            return vec![(Direction::South, index)];
        } else if direction == Direction::South {
            return vec![(Direction::East, index)];
        } else if direction == Direction::West {
            return vec![(Direction::North, index)];
        }
    }

    vec![]
}

fn one(map: &Map<char>) -> usize {
    let mut directions: Vec<(Direction, usize)> = next(map, 0, Direction::East);
    let mut energized: Vec<bool> = vec![false; map.tiles.len()];
    let mut visited: HashSet<(Direction, usize)> = HashSet::new();
    while let Some((direction, index)) = directions.pop() {
        if let Some(next_index) = map.move_from(index, &direction) {
            energized[next_index] = true;
            if !visited.contains(&(direction.clone(), next_index)) {
                visited.insert((direction.clone(), next_index));
                directions.extend(next(map, next_index, direction));
            }
        }
    }
    energized.iter().filter(|e| **e).count()
}

fn two() -> usize {
    0
}

pub fn run(runner: &Runner) {
    let map: Map<char> = get_non_empty_lines(&runner.path).fold(
        Map {
            tiles: vec![],
            line_length: 0,
        },
        |mut map, line| {
            let row = line.chars().collect::<Vec<char>>();
            map.line_length = row.len();
            map.tiles.extend(row);
            map
        },
    );
    let result = match runner.part {
        Part::One => one(&map),
        Part::Two => two(),
    };

    println!("result {}", result);
}
