use std::{
    fmt::{Display, Formatter},
    usize,
};

use crate::{
    utils::{get_non_empty_lines, map::Map, unique_permutations},
    Part, Runner,
};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Tile {
    Empty,
    Galaxy,
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

    let permutations: Vec<Vec<usize>> = unique_permutations(&galaxies);
    permutations.iter().map(|p| map.distance(p[0], p[1])).sum()
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Galaxy {
    col: usize,
    row: usize,
}

fn two(map: Map<Tile>) -> usize {
    let n = 1_000_000;
    let mut galaxies: Vec<Galaxy> = map
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| !tile.is_empty())
        .map(|(index, _)| {
            let (col, row) = map.to_xy(index);
            Galaxy { col, row }
        })
        .collect();

    let emty_rows: Vec<usize> = map
        .get_rows()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|tile| tile.is_empty()))
        .map(|(index, _)| index)
        .collect();

    let empty_columns: Vec<usize> = map
        .get_columns()
        .enumerate()
        .filter(|(_, column)| column.iter().all(|tile| tile.is_empty()))
        .map(|(index, _)| index)
        .collect();

    galaxies
        .iter_mut()
        .map(|galaxy| {
            let empty_rows_count = emty_rows.iter().filter(|row| row < &&galaxy.row).count();
            let empty_columns_count = empty_columns.iter().filter(|c| c < &&galaxy.col).count();
            galaxy.row += empty_rows_count * (n - 1);
            galaxy.col += empty_columns_count * (n - 1);
        })
        .for_each(drop);

    unique_permutations(&galaxies)
        .iter()
        .map(|p| get_distance(&p[0], &p[1]))
        .sum()
}

fn get_distance(g1: &Galaxy, g2: &Galaxy) -> usize {
    ((g1.col as isize - g2.col as isize).abs() + (g1.row as isize - g2.row as isize).abs()) as usize
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
    fn test_triple_empty_rows() {
        let mut map: Map<Tile> = "#..\n...\n..#".into();
        multiply_empty_rows(&mut map, 3);
        assert_eq!(map.to_string(), "#..\n...\n...\n...\n..#");
    }

    #[test]
    fn test_double_empty_columns() {
        let mut map: Map<Tile> = ".#.#\n.#.#\n#..#".into();
        multiply_empty_columns(&mut map, 2);
        assert_eq!(map.to_string(), ".#..#\n.#..#\n#...#");
    }

    #[test]
    fn test_triple_empty_columns() {
        let mut map: Map<Tile> = ".#.#\n.#.#\n#..#".into();
        multiply_empty_columns(&mut map, 3);
        assert_eq!(map.to_string(), ".#...#\n.#...#\n#....#");
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
