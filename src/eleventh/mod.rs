use std::{
    fmt::{Display, Formatter},
    usize,
};

use nanoid::nanoid;

use crate::{
    utils::{get_non_empty_lines, map::Map},
    Part, Runner,
};

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Galaxy(String),
}

impl Tile {
    fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Galaxy(_) => write!(f, "#"),
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy(nanoid!()),
            _ => panic!("invalid tile"),
        }
    }
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|c| c.into()).collect()
}

fn multiply_empty_rows(map: &mut Map<Tile>, n: usize) {
    let chunks = map
        .get_rows()
        .flat_map(|x| {
            if x.iter().all(|tile| tile.is_empty()) {
                vec![vec![Tile::Empty; map.line_length]; n]
            } else {
                vec![x.to_vec()]
            }
        })
        .collect();
    map.replace_rows(chunks);
}

fn multiply_empty_columns(map: &mut Map<Tile>, n: usize) {
    let chunks: Vec<Vec<Tile>> = map
        .get_columns()
        .flat_map(|x| {
            if x.iter().all(|tile| tile.is_empty()) {
                vec![vec![Tile::Empty; x.len()]; n]
            } else {
                vec![x.to_vec()]
            }
        })
        .collect();

    map.replace_columns(chunks);
}

fn one(mut map: Map<Tile>) -> usize {
    multiply_empty_rows(&mut map, 2);
    multiply_empty_columns(&mut map, 2);

    let galaxies: Vec<usize> = map
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| !tile.is_empty())
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

fn two(mut map: Map<Tile>) -> usize {
    multiply_empty_rows(&mut map, 1_000_000);
    multiply_empty_columns(&mut map, 1_000_000);

    let galaxies: Vec<usize> = map
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| !tile.is_empty())
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

pub fn run(runner: &Runner) {
    let mut line_length: usize = 0;
    let tiles: Vec<Tile> = get_non_empty_lines(&runner.path)
        .flat_map(|line| {
            line_length = line.trim().len();
            parse_line(line.trim())
        })
        .collect();

    let map = Map { tiles, line_length };

    let result = match runner.part {
        Part::One => one(map),
        Part::Two => two(map),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_double_empty_rows() {
        let mut map: Map<Tile> = "#..\n...\n..#".into();
        multiply_empty_rows(&mut map, 2);
        assert_eq!(map.to_string(), "#..\n...\n...\n..#");
    }

    #[test]
    fn test_double_empty_columns() {
        let mut map: Map<Tile> = ".#.#\n.#.#\n#..#".into();
        multiply_empty_columns(&mut map, 2);
        assert_eq!(map.to_string(), ".#..#\n.#..#\n#...#");
    }

    #[test]
    fn test_one() {
        let map: Map<Tile> = "#..\n...\n..#".into();
        assert_eq!(one(map), 6);
    }

    #[test]
    fn test_one_bigger() {
        let map: Map<Tile> = ".#..\n#..#\n...#\n....".into();
        assert_eq!(one(map), 21);
    }

    #[test]
    fn test_input() {
        let input: Map<Tile> = "
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

        assert_eq!(one(input), 374);
    }
}
