use std::{collections::HashMap, usize};

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

fn next(
    map: &Map<usize>,
    path: Path,
    cache: &mut HashMap<([Option<Direction>; 3], usize), Option<usize>>,
    current_best: &mut usize,
    current_index: usize,
) -> Option<usize> {
    if let Some(res) = cache.get(&(path.directions.clone(), current_index)) {
        return *res;
    }

    let new_current_best = path.indexes.iter().map(|i| map.tiles[*i]).sum::<usize>();

    if new_current_best > *current_best {
        return None;
    }

    if current_index == map.tiles.len() - 1 {
        if new_current_best < *current_best {
            *current_best = new_current_best;
        }

        return Some(map.tiles[current_index]);
    }

    let mut val = None;

    for d in &DIRECTIONS {
        if Some(d.opposite()) == path.last_direction().cloned() {
            continue;
        }
        if path.need_to_turn && Some(d) == path.last_direction() {
            continue;
        }
        if let Some(i) = map.move_from(current_index, d) {
            if !path.contains(i) {
                let mut path = path.clone();
                path.push_direction(d.clone());
                path.push_index(i);
                if let Some(v) = next(map, path, cache, current_best, i) {
                    let v = v + if current_index == 0 {
                        0
                    } else {
                        map.tiles[current_index]
                    };
                    if val.is_none() || v < val.unwrap() {
                        val = Some(v);
                    }
                }
            }
        }
    }

    cache.insert((path.directions, current_index), val);
    val
}

fn one(map: &Map<usize>) -> usize {
    let mut cache: HashMap<([Option<Direction>; 3], usize), Option<usize>> = HashMap::new();
    let mut current_best = usize::MAX;
    next(map, Path::new(), &mut cache, &mut current_best, 0).unwrap()
}

fn two(_map: &Map<usize>) -> usize {
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
