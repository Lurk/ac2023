use std::{fmt::Display, usize};

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
    Left,
    Right,
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

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => '.',
            Tile::Start => 'S',
            Tile::Pipe(Direction::North, Direction::South) => '|',
            Tile::Pipe(Direction::West, Direction::East) => '-',
            Tile::Pipe(Direction::North, Direction::East) => 'L',
            Tile::Pipe(Direction::North, Direction::West) => 'J',
            Tile::Pipe(Direction::West, Direction::South) => '7',
            Tile::Pipe(Direction::East, Direction::South) => 'F',
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
            Tile::Left => write!(f, "1"),
            Tile::Right => write!(f, "2"),
            _ => panic!("Invalid tile"),
        }
    }
}

fn get_loop(tiles: &[Tile], line_length: usize) -> Vec<usize> {
    let mut current_position = tiles.iter().position(|tile| tile == &Tile::Start).unwrap();
    let mut directions = Vec::from(DIRECTIONS);
    let mut loop_positions = vec![current_position];

    loop {
        for direction in directions.iter() {
            if let Some(pos) = direction.get_index(tiles.len(), line_length, current_position) {
                match &tiles[pos] {
                    Tile::Pipe(dir1, dir2) => {
                        if dir1.get_index(tiles.len(), line_length, pos) == Some(current_position) {
                            current_position = pos;
                            directions = vec![dir2.clone()];
                            loop_positions.push(pos);
                            break;
                        } else if dir2.get_index(tiles.len(), line_length, pos)
                            == Some(current_position)
                        {
                            current_position = pos;
                            directions = vec![dir1.clone()];
                            loop_positions.push(pos);
                            break;
                        }
                    }
                    Tile::Empty => continue,
                    Tile::Start => return loop_positions,
                    _ => panic!("Invalid tile"),
                }
            }
        }
    }
}

fn one(tiles: &[Tile], line_length: usize) -> usize {
    let loop_positions = get_loop(tiles, line_length);
    loop_positions.len() / 2
}

fn two(tiles: &[Tile], line_length: usize) -> usize {
    let loop_positions = get_loop(tiles, line_length);
    let mut tiles: Vec<Tile> = tiles
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if loop_positions.contains(&i) {
                x.clone()
            } else {
                Tile::Empty
            }
        })
        .collect();

    let mut previous_position = loop_positions[0];
    let mut to_check: (Vec<Direction>, Vec<Direction>);
    for position in loop_positions.iter().skip(1) {
        if position > &previous_position && position - previous_position == 1 {
            // East - J 7
            to_check = match tiles[*position] {
                Tile::Pipe(Direction::West, Direction::East) => {
                    (vec![Direction::North], vec![Direction::South])
                }
                Tile::Pipe(Direction::North, Direction::West) => {
                    (vec![], vec![Direction::East, Direction::South])
                }
                Tile::Pipe(Direction::West, Direction::South) => {
                    (vec![Direction::North, Direction::East], vec![])
                }
                _ => panic!("Invalid tile going east"),
            };
        } else if position > &previous_position && position - previous_position == line_length {
            // South | L J
            to_check = match tiles[*position] {
                Tile::Pipe(Direction::North, Direction::South) => {
                    (vec![Direction::East], vec![Direction::West])
                }
                Tile::Pipe(Direction::North, Direction::East) => {
                    (vec![], vec![Direction::South, Direction::West])
                }
                Tile::Pipe(Direction::North, Direction::West) => {
                    (vec![Direction::South, Direction::East], vec![])
                }
                _ => panic!("Invalid tile going south"),
            };
        } else if &previous_position > position && previous_position - position == 1 {
            // West - L F
            to_check = match tiles[*position] {
                Tile::Pipe(Direction::West, Direction::East) => {
                    (vec![Direction::South], vec![Direction::North])
                }
                Tile::Pipe(Direction::North, Direction::East) => {
                    (vec![Direction::North, Direction::West], vec![])
                }
                Tile::Pipe(Direction::East, Direction::South) => {
                    (vec![], vec![Direction::North, Direction::West])
                }
                _ => panic!("Invalid tile going west"),
            };
        } else if &previous_position > position && previous_position - position == line_length {
            // North | 7 F
            to_check = match tiles[*position] {
                Tile::Pipe(Direction::North, Direction::South) => {
                    (vec![Direction::West], vec![Direction::East])
                }
                Tile::Pipe(Direction::West, Direction::South) => {
                    (vec![], vec![Direction::North, Direction::East])
                }
                Tile::Pipe(Direction::East, Direction::South) => {
                    (vec![Direction::North, Direction::West], vec![])
                }
                _ => panic!("Invalid tile going north"),
            };
        } else {
            panic!("Invalid loop");
        }

        previous_position = *position;

        for direction in to_check.0.iter() {
            if let Some(pos) = direction.get_index(tiles.len(), line_length, *position) {
                if tiles[pos] == Tile::Empty {
                    tiles[pos] = Tile::Left;
                }
            }
        }

        for direction in to_check.1.iter() {
            if let Some(pos) = direction.get_index(tiles.len(), line_length, *position) {
                if tiles[pos] == Tile::Empty {
                    tiles[pos] = Tile::Right;
                }
            }
        }
    }

    for index in 0..tiles.len() {
        if tiles[index] == Tile::Right {
            for direction in DIRECTIONS.iter() {
                if let Some(pos) = direction.get_index(tiles.len(), line_length, index) {
                    if tiles[pos] == Tile::Empty {
                        tiles[pos] = Tile::Right;
                    }
                }
            }
        }
    }

    tiles.iter().filter(|x| x == &&Tile::Right).count()
}

fn parse_line(str: &str) -> Vec<Tile> {
    str.trim_end().chars().map(Tile::from).collect::<Vec<_>>()
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
        Part::Two => two(&tiles, line_length),
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
