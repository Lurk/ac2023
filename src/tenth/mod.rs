use std::fmt::Display;

use crate::{utils::get_non_empty_lines, Part, Runner};

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_index(&self, total_len: usize, line_length: usize, index: usize) -> Option<usize> {
        match self {
            Direction::North => {
                if index >= line_length {
                    Some(index - line_length)
                } else {
                    None
                }
            }
            Direction::East => {
                if index % line_length != line_length - 1 {
                    Some(index + 1)
                } else {
                    None
                }
            }
            Direction::South => {
                if index < total_len - line_length {
                    Some(index + line_length)
                } else {
                    None
                }
            }
            Direction::West => {
                if index % line_length != 0 {
                    Some(index - 1)
                } else {
                    None
                }
            }
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Start,
    Pipe(Direction, Direction),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::Pipe(Direction::North, Direction::South),
            '-' => Self::Pipe(Direction::West, Direction::East),
            'L' => Self::Pipe(Direction::North, Direction::East),
            'J' => Self::Pipe(Direction::North, Direction::West),
            '7' => Self::Pipe(Direction::West, Direction::South),
            'F' => Self::Pipe(Direction::East, Direction::South),
            '.' => Self::Empty,
            'S' => Self::Start,

            _ => panic!("Invalid tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Start => write!(f, "S"),
            Tile::Pipe(Direction::North, Direction::South) => write!(f, "|"),
            Tile::Pipe(Direction::West, Direction::East) => write!(f, "-"),
            Tile::Pipe(Direction::North, Direction::East) => write!(f, "L"),
            Tile::Pipe(Direction::North, Direction::West) => write!(f, "J"),
            Tile::Pipe(Direction::West, Direction::South) => write!(f, "7"),
            Tile::Pipe(Direction::East, Direction::South) => write!(f, "F"),
            _ => panic!("Invalid tile"),
        }
    }
}

fn one(tiles: &[Tile], line_length: usize) -> usize {
    let mut current_position = tiles.iter().position(|tile| tile == &Tile::Start).unwrap();
    let mut steps = 0;
    let mut directions = Vec::from(DIRECTIONS);

    loop {
        for direction in directions.iter() {
            if let Some(pos) = direction.get_index(tiles.len(), line_length, current_position) {
                match &tiles[pos] {
                    Tile::Pipe(dir1, dir2) => {
                        if dir1.get_index(tiles.len(), line_length, pos) == Some(current_position) {
                            current_position = pos;
                            directions = vec![dir2.clone()];
                            steps += 1;
                            break;
                        } else if dir2.get_index(tiles.len(), line_length, pos)
                            == Some(current_position)
                        {
                            current_position = pos;
                            directions = vec![dir1.clone()];
                            steps += 1;
                            println!("{}: {}", steps, current_position);
                            break;
                        }
                    }
                    Tile::Empty => continue,
                    Tile::Start => return (steps + 1) / 2,
                }
            }
        }
    }
}

fn two() -> usize {
    0
}

fn parse_line(str: &str) -> Vec<Tile> {
    str.trim_end()
        .chars()
        .map(|c| Tile::from(c))
        .collect::<Vec<_>>()
}

pub fn run(runner: &Runner) {
    let mut line_length = 0;
    let tiles: Vec<Tile> = get_non_empty_lines(&runner.path)
        .flat_map(|str| {
            line_length = str.len();
            parse_line(&str)
        })
        .collect();

    let result = match runner.part {
        Part::One => one(&tiles, line_length),
        Part::Two => two(),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {

    use super::Direction;

    #[test]
    fn direction_get_index() {
        assert_eq!(Direction::North.get_index(9, 3, 4), Some(1));
        assert_eq!(Direction::East.get_index(9, 3, 4), Some(5));
        assert_eq!(Direction::South.get_index(9, 3, 4), Some(7));
        assert_eq!(Direction::West.get_index(9, 3, 4), Some(3));
        assert_eq!(Direction::North.get_index(9, 3, 1), None);
    }
}
