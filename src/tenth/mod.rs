use std::{fmt::Display, usize};

use crate::{
    utils::{direction::Direction, get_non_empty_lines},
    Part, Runner,
};

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Start,
    Pipe(Direction, Direction),
    Left,
    Right,
}

impl Tile {
    fn to_left_right(&self, direction: Direction) -> (Vec<Direction>, Vec<Direction>) {
        match direction {
            Direction::East => match self {
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
            },
            Direction::South => match self {
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
            },
            Direction::West => match self {
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
            },
            Direction::North => match self {
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
            },
            _ => panic!("Invalid direction"),
        }
    }
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
            Tile::Left => write!(f, "1"),
            Tile::Right => write!(f, "2"),
            _ => panic!("Invalid tile"),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

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

fn two(original_tiles: &[Tile], line_length: usize) -> usize {
    let loop_positions = get_loop(original_tiles, line_length);
    let mut tiles: Vec<Tile> = vec![Tile::Empty; original_tiles.len()];
    loop_positions
        .iter()
        .for_each(|x| tiles[*x] = original_tiles[*x].clone());

    let mut previous_position = loop_positions[0];
    for position in loop_positions.iter().skip(1) {
        let to_check = if position > &previous_position && position - previous_position == 1 {
            tiles[*position].to_left_right(Direction::East)
        } else if position > &previous_position && position - previous_position == line_length {
            tiles[*position].to_left_right(Direction::South)
        } else if &previous_position > position && previous_position - position == 1 {
            tiles[*position].to_left_right(Direction::West)
        } else if &previous_position > position && previous_position - position == line_length {
            tiles[*position].to_left_right(Direction::North)
        } else {
            panic!("Invalid loop");
        };

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
