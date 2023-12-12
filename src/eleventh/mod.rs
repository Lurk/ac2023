use std::{
    fmt::{Display, Formatter},
    usize,
};

use crate::{
    utils::{get_non_empty_lines, map::Map},
    Part, Runner,
};

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Galaxy,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Galaxy => write!(f, "#"),
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy,
            _ => panic!("invalid tile"),
        }
    }
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|c| c.into()).collect()
}

fn get_empty_rows_numbers(map: &Map<Tile>) -> Vec<usize> {
    map.get_rows()
        .enumerate()
        .fold(Vec::new(), |mut acc: Vec<usize>, (index, row)| {
            if row.iter().all(|tile| *tile == Tile::Empty) {
                acc.push(index);
            }
            acc
        })
}

fn double_empty_rows(map: &mut Map<Tile>) {
    let rows = get_empty_rows_numbers(map);
    let chunks: Vec<Vec<Tile>> = map
        .get_rows()
        .enumerate()
        .flat_map(|(i, x)| {
            if rows.contains(&i) {
                vec![x.to_vec(), vec![Tile::Empty; map.line_length]]
            } else {
                vec![x.to_vec()]
            }
        })
        .collect();

    map.tiles.clear();
    map.tiles.extend(chunks.iter().flatten().cloned());
}

fn get_empty_columns_numbers(map: &Map<Tile>) -> Vec<usize> {
    map.get_columns()
        .enumerate()
        .fold(Vec::new(), |mut acc: Vec<usize>, (index, column)| {
            if column.iter().all(|tile| *tile == Tile::Empty) {
                acc.push(index);
            }
            acc
        })
}

fn double_empty_columns(map: &mut Map<Tile>) {
    let columns = get_empty_columns_numbers(map);
    let mut chunks: Vec<Vec<Tile>> = map.get_columns().collect();
    for (i, column) in columns.iter().enumerate() {
        chunks.insert(*column + i, vec![Tile::Empty; chunks[0].len()]);
    }
    let mut tiles = Vec::new();
    for i in 0..chunks[0].len() {
        for chunk in &chunks {
            tiles.push(chunk[i].clone());
        }
    }

    map.tiles.clear();
    map.tiles.extend(tiles);
    map.line_length += columns.len();
}

fn one(map: &Map<Tile>) -> usize {
    let galaxies: Vec<usize> = map
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| **tile == Tile::Galaxy)
        .map(|(index, _)| index)
        .collect();
    let mut permutations: Vec<Vec<usize>> = Vec::new();
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            permutations.push(vec![galaxies[i], galaxies[j]]);
        }
    }

    permutations.iter().map(|p| map.distance(p[0], p[1])).sum()
}

fn two() -> usize {
    0
}

pub fn run(runner: &Runner) {
    let mut line_length: usize = 0;
    let tiles: Vec<Tile> = get_non_empty_lines(&runner.path)
        .flat_map(|line| {
            line_length = line.trim().len();
            parse_line(line.trim())
        })
        .collect();

    let mut map = Map { tiles, line_length };
    double_empty_rows(&mut map);
    double_empty_columns(&mut map);

    let result = match runner.part {
        Part::One => one(&map),
        Part::Two => two(),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_get_empty_rows_numbers() {
        let map: Map<Tile> = "...\n###\n...".into();
        let empty_rows = get_empty_rows_numbers(&map);
        assert_eq!(empty_rows, vec![0, 2]);
    }

    #[test]
    fn test_double_empty_rows() {
        let mut map: Map<Tile> = "#..\n...\n..#".into();
        double_empty_rows(&mut map);
        assert_eq!(map.to_string(), "#..\n...\n...\n..#");
    }

    #[test]
    fn test_get_empty_columns_numbers() {
        let map: Map<Tile> = "#..\n...\n..#".into();
        let empty_columns = get_empty_columns_numbers(&map);
        assert_eq!(empty_columns, vec![1]);
    }

    #[test]
    fn test_double_empty_columns() {
        let mut map: Map<Tile> = ".#.#\n.#.#\n#..#".into();
        double_empty_columns(&mut map);
        assert_eq!(map.to_string(), ".#..#\n.#..#\n#...#");
    }

    #[test]
    fn test_one() {
        let map: Map<Tile> = "#..\n...\n..#".into();
        assert_eq!(one(&map), 4);
    }

    #[test]
    fn test_one_bigger() {
        let map: Map<Tile> = ".#..\n#..#\n...#\n....".into();
        assert_eq!(one(&map), 17);
    }

    #[test]
    fn test_input() {
        let mut input: Map<Tile> = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .into();

        let larger: Map<Tile> = "
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
            .into();

        double_empty_rows(&mut input);
        double_empty_columns(&mut input);

        assert_eq!(input.to_string(), larger.to_string());

        assert_eq!(one(&input), 374);
    }
}
