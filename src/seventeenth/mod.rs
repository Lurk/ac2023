use std::{collections::HashMap, sync::Mutex, usize};

use rayon::prelude::*;

use crate::{
    utils::{direction::Direction, get_non_empty_lines, map::Map},
    Part, Runner,
};

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

#[derive(Clone)]
struct Path {
    pub need_to_turn: bool,
    pub indexes: Vec<usize>,
    direction_repeated_times: usize,
    directions: [Option<Direction>; 3],
}

impl Path {
    fn new() -> Self {
        Self {
            need_to_turn: false,
            direction_repeated_times: 0,
            indexes: vec![],
            directions: [None, None, None],
        }
    }

    fn push_direction(&mut self, direction: Direction) {
        if self.directions[2].as_ref() == Some(&direction) {
            self.direction_repeated_times += 1;
        } else {
            self.direction_repeated_times = 1;
        }
        self.need_to_turn = self.direction_repeated_times >= 3;
        self.directions.rotate_left(1);
        self.directions[2] = Some(direction);
    }

    fn last_direction(&self) -> Option<&Direction> {
        self.directions[2].as_ref()
    }

    fn contains(&self, index: usize) -> bool {
        self.indexes.contains(&index)
    }

    fn push_index(&mut self, index: usize) {
        self.indexes.push(index);
    }
}

fn debug(map: &Map<usize>, path: &Path) {
    let mut map = map.clone();
    for i in path.indexes.iter() {
        map.tiles[*i] = 0;
    }
    println!("{}", map);
}

fn next(
    map: &Map<usize>,
    path: Path,
    cache: &mut HashMap<([Option<Direction>; 3], usize), Option<usize>>,
    current_index: usize,
) -> Option<usize> {
    if current_index == map.tiles.len() - 1 {
        return Some(map.tiles[current_index]);
    }

    let mut possible_directions = DIRECTIONS.to_vec();
    if let Some(direction) = path.last_direction() {
        possible_directions.retain(|d| d.opposite() != *direction);
        if path.need_to_turn {
            possible_directions.retain(|d| d != direction);
        }
    }

    if cache.contains_key(&(path.directions.clone(), current_index)) {
        return cache
            .get(&(path.directions, current_index))
            .unwrap()
            .clone();
    }

    let val = possible_directions
        .iter()
        .filter_map(|d| map.move_from(current_index, d).map(|i| (d, i)))
        .filter(|(_, i)| !path.contains(*i))
        .filter_map(|(d, i)| {
            let mut path = path.clone();
            path.push_direction(d.clone());
            path.push_index(i);

            next(map, path, cache, i).map(|v| {
                v + if current_index == 0 {
                    0
                } else {
                    map.tiles[current_index]
                }
            })
        })
        .min();

    cache.insert((path.directions, current_index), val);
    val
}

fn one(map: &Map<usize>) -> usize {
    let mut cache: HashMap<([Option<Direction>; 3], usize), Option<usize>> = HashMap::new();
    next(map, Path::new(), &mut cache, 0).unwrap()
}

fn two(map: &Map<usize>) -> usize {
    0
}

fn lines_to_map(lines: impl Iterator<Item = String>) -> Map<usize> {
    lines.fold(
        Map {
            tiles: vec![],
            line_length: 0,
        },
        |mut map, line| {
            let row = line
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            map.line_length = row.len();
            map.tiles.extend(row);
            map
        },
    )
}

pub fn run(runner: &Runner) {
    let map = lines_to_map(get_non_empty_lines(&runner.path));

    let result = match runner.part {
        Part::One => one(&map),
        Part::Two => two(&map),
    };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    #[test]
    fn test_one() {
        let map = lines_to_map(TEST_INPUT.lines().map(String::from));
        assert_eq!(one(&map), 102);
    }
}
