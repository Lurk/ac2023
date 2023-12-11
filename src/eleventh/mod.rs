use std::{
    fmt::{Display, Formatter},
    usize,
};

use crate::{
    utils::{direction::Direction, get_non_empty_lines},
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

fn get_empty_rows_numbers(tiles: &[Tile], line_length: usize) -> Vec<usize> {
    tiles
        .chunks(line_length)
        .enumerate()
        .fold(Vec::new(), |mut acc: Vec<usize>, (index, chunk)| {
            if chunk.iter().all(|tile| *tile == Tile::Empty) {
                acc.push(index);
            }
            acc
        })
}

fn double_empty_rows(tiles: &mut Vec<Tile>, line_length: usize) -> usize {
    let rows = get_empty_rows_numbers(&tiles, line_length);
    for row in rows.iter() {
        let i = (row + 1) * line_length;
        tiles
            .splice(i..i, vec![Tile::Empty; line_length].iter().cloned())
            .for_each(drop);
    }
    rows.len()
}

fn get_empty_columns_numbers(tiles: &[Tile], line_length: usize) -> Vec<usize> {
    let mut empty_columns: Vec<usize> = Vec::new();
    for i in 0..line_length {
        let mut current = i;
        if tiles[i] == Tile::Empty {
            let mut all = true;
            while let Some(pos) = Direction::South.get_index(tiles.len(), line_length, current) {
                current = pos;
                if tiles[pos] != Tile::Empty {
                    all = false;
                    break;
                }
            }
            if all {
                empty_columns.push(i);
            }
        }
    }
    empty_columns
}

fn double_empty_columns(tiles: &mut Vec<Tile>, line_length: usize) -> usize {
    let columns = get_empty_columns_numbers(&tiles, line_length);
    println!("columns: {:?}", columns);
    let mut chunks: Vec<Vec<Tile>> = tiles.chunks(line_length).map(|x| x.to_vec()).collect();
    for column in columns.iter() {
        for chunk in chunks.iter_mut() {
            chunk.insert(*column, Tile::Empty);
        }
    }

    let ridiculous: Vec<Tile> = chunks.iter().cloned().flat_map(|x| x).collect();
    tiles.clear();
    tiles.extend(ridiculous);

    columns.len()
}

fn distance(a: usize, b: usize, line_length: usize) -> usize {
    let a_x = a % line_length;
    let a_y = a / line_length;
    let b_x = b % line_length;
    let b_y = b / line_length;
    ((a_x as isize - b_x as isize).abs() + (a_y as isize - b_y as isize).abs()) as usize
}

fn one(map: &[Tile], line_length: usize) -> usize {
    let galaxies: Vec<usize> = map
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

    permutations
        .iter()
        .map(|p| distance(p[0], p[1], line_length))
        .sum()
}

fn two() -> usize {
    0
}

pub fn run(runner: &Runner) {
    let mut line_length: usize = 0;
    let mut map: Vec<Tile> = get_non_empty_lines(&runner.path)
        .flat_map(|line| {
            line_length = line.trim().len();
            parse_line(line.trim())
        })
        .collect();

    double_empty_rows(&mut map, line_length);
    line_length += double_empty_columns(&mut map, line_length);

    let result = match runner.part {
        Part::One => one(&map, line_length),
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
        #[rustfmt::skip]
        let tiles: Vec<Tile> = vec![
            Tile::Empty, Tile::Empty,  Tile::Empty,
            Tile::Galaxy,Tile::Galaxy, Tile::Galaxy,
            Tile::Empty, Tile::Empty,  Tile::Empty,
        ];
        let line_length = 3;
        let empty_rows = get_empty_rows_numbers(&tiles, line_length);
        assert_eq!(empty_rows, vec![0, 2]);
    }

    #[test]
    fn test_double_empty_rows() {
        #[rustfmt::skip]
        let mut tiles: Vec<Tile> = vec![
            Tile::Galaxy, Tile::Empty, Tile::Empty,
            Tile::Empty,Tile::Empty,Tile::Empty,
            Tile::Empty, Tile::Empty, Tile::Galaxy,
        ];
        let line_length = 3;
        double_empty_rows(&mut tiles, line_length);
        #[rustfmt::skip]
        assert_eq!(
            tiles,
            vec![
                Tile::Galaxy, Tile::Empty, Tile::Empty,
                Tile::Empty, Tile::Empty, Tile::Empty,
                Tile::Empty, Tile::Empty, Tile::Empty,
                Tile::Empty, Tile::Empty, Tile::Galaxy,
            ]
        );
    }

    #[test]
    fn test_get_empty_columns_numbers() {
        #[rustfmt::skip]
        let tiles: Vec<Tile> = vec![
            Tile::Galaxy, Tile::Empty, Tile::Empty,
            Tile::Empty,Tile::Empty, Tile::Empty,
            Tile::Empty, Tile::Empty, Tile::Galaxy,
        ];
        let line_length = 3;
        let empty_columns = get_empty_columns_numbers(&tiles, line_length);
        assert_eq!(empty_columns, vec![1]);
    }

    #[test]
    fn test_double_empty_columns() {
        #[rustfmt::skip]
        let mut tiles: Vec<Tile> = vec![
            Tile::Empty, Tile::Galaxy, Tile::Empty, Tile::Galaxy,
            Tile::Empty,Tile::Galaxy, Tile::Empty, Tile::Galaxy,
            Tile::Galaxy, Tile::Empty, Tile::Empty, Tile::Galaxy,
        ];
        let line_length = 4;
        double_empty_columns(&mut tiles, line_length);
        #[rustfmt::skip]
        assert_eq!(
            map_to_str(&tiles, 5),
            map_to_str(&[
                Tile::Empty, Tile::Galaxy, Tile::Empty, Tile::Empty, Tile::Galaxy,
                Tile::Empty,Tile::Galaxy, Tile::Empty, Tile::Empty, Tile::Galaxy,
                Tile::Galaxy, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Galaxy,
            ], 5)
        );
    }

    #[test]
    fn test_one() {
        #[rustfmt::skip]
        let tiles: Vec<Tile> = vec![
            Tile::Galaxy, Tile::Empty, Tile::Empty,
            Tile::Empty,Tile::Empty, Tile::Empty,
            Tile::Galaxy, Tile::Empty, Tile::Galaxy,
        ];

        assert_eq!(one(&tiles, 3), 8);
    }

    #[test]
    fn test_one_bigger() {
        #[rustfmt::skip]
        let tiles: Vec<Tile> =  vec![
            Tile::Empty, Tile::Galaxy, Tile::Empty, Tile::Empty,
            Tile::Galaxy,Tile::Empty, Tile::Empty, Tile::Galaxy,
            Tile::Empty, Tile::Empty, Tile::Empty, Tile::Galaxy,
            Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
        ];

        assert_eq!(one(&tiles, 4), 17);
    }

    fn map_to_str(map: &[Tile], line_length: usize) -> String {
        map.chunks(line_length)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|tile| format!("{}", tile))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn str_to_tiles(input: &str) -> (Vec<Tile>, usize) {
        let mut line_length: usize = 0;
        let tiles: Vec<Tile> = input
            .split('\n')
            .flat_map(|line| {
                line_length = line.trim().len();
                parse_line(line.trim())
            })
            .collect();
        (tiles, line_length)
    }

    #[test]
    fn test_input() {
        let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let larger = "
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
#....#.......";

        let (mut map, mut line_length) = str_to_tiles(input);
        println!("line_length: {}", line_length);
        double_empty_rows(&mut map, line_length);
        line_length += double_empty_columns(&mut map, line_length);

        assert_eq!(map_to_str(&map, line_length), larger);

        assert_eq!(one(&map, line_length), 374);
    }
}
