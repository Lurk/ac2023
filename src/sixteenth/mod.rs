use std::{cmp::max, collections::HashSet, usize};

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

fn count_energized(map: &Map<char>, start: &mut Vec<(Direction, usize)>) -> usize {
    let mut energized: Vec<bool> = vec![false; map.tiles.len()];
    let mut visited: HashSet<(Direction, usize)> = HashSet::new();
    while let Some((direction, index)) = start.pop() {
        energized[index] = true;
        if let Some(next_index) = map.move_from(index, &direction) {
            if !visited.contains(&(direction.clone(), next_index)) {
                visited.insert((direction.clone(), next_index));
                start.extend(next(map, next_index, direction));
            }
        }
    }
    energized.iter().filter(|e| **e).count()
}

fn one(map: &Map<char>) -> usize {
    let mut directions: Vec<(Direction, usize)> = next(map, 0, Direction::East);
    count_energized(map, &mut directions)
}

fn two(map: &Map<char>) -> usize {
    let south = (0..map.line_length)
        .map(|i| {
            let mut directions: Vec<(Direction, usize)> = next(map, i, Direction::South);
            count_energized(map, &mut directions)
        })
        .max()
        .unwrap();

    let north = (map.tiles.len() - map.line_length..map.tiles.len())
        .map(|i| {
            let mut directions: Vec<(Direction, usize)> = next(map, i, Direction::North);
            count_energized(map, &mut directions)
        })
        .max()
        .unwrap();

    let east = (0..map.tiles.len())
        .step_by(map.line_length)
        .map(|i| {
            let mut directions: Vec<(Direction, usize)> = next(map, i, Direction::East);
            count_energized(map, &mut directions)
        })
        .max()
        .unwrap();

    let west = (map.line_length - 1..map.tiles.len())
        .step_by(map.line_length)
        .map(|i| {
            let mut directions: Vec<(Direction, usize)> = next(map, i, Direction::West);
            count_energized(map, &mut directions)
        })
        .max()
        .unwrap();

    max(south, max(north, max(east, west)))
}

fn lines_to_map(lines: impl Iterator<Item = String>) -> Map<char> {
    lines.fold(
        Map {
            tiles: vec![],
            line_length: 0,
        },
        |mut map, line| {
            let row = line.trim().chars().collect::<Vec<char>>();
            map.line_length = row.len();
            map.tiles.extend(row);
            map
        },
    )
}

pub fn run(runner: &Runner) {
    let map: Map<char> = lines_to_map(get_non_empty_lines(&runner.path));
    let result = match runner.part {
        Part::One => one(&map),
        Part::Two => two(&map),
    };

    println!("result {}", result);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_one() {
        let map: Map<char> = lines_to_map(TEST_INPUT.lines().map(|x| x.to_string()));
        assert_eq!(one(&map), 46);
    }
}
