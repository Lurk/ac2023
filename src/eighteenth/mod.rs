use crate::{
    utils::{direction::Direction, get_non_empty_lines},
    Part, Runner,
};

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    amount_of_steps: i64,
}

impl Instruction {
    pub fn from_color(mut color: String) -> Self {
        let d = color.pop().expect("should have a direction");
        Instruction {
            direction: match d {
                '0' => Direction::East,
                '1' => Direction::South,
                '2' => Direction::West,
                '3' => Direction::North,
                _ => panic!("Invalid direction"),
            },
            amount_of_steps: i64::from_str_radix(color.as_str(), 16).expect("should be a number"),
        }
    }
}

impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        let chunks: Vec<&str> = val.trim().split(' ').collect();
        Instruction {
            direction: match chunks[0] {
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                "U" => Direction::North,
                _ => panic!("Invalid direction"),
            },
            amount_of_steps: chunks[1].parse().unwrap(),
        }
    }
}

impl From<String> for Instruction {
    fn from(val: String) -> Self {
        Instruction::from(val.as_str())
    }
}

fn shoelace_area_of_polygon(edges: &[(i64, i64)]) -> i64 {
    edges
        .windows(2)
        .fold(0, |acc, edge| {
            let (x1, y1) = edge[0];
            let (x2, y2) = edge[1];
            acc + x1 * y2 - x2 * y1
        })
        .abs()
        / 2
}

fn instructions_to_area(instructions: impl Iterator<Item = Instruction>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut len = 0;
    let mut edges: Vec<(i64, i64)> = vec![(x, y)];
    for instruction in instructions {
        len += instruction.amount_of_steps;
        match instruction.direction {
            Direction::East => x += instruction.amount_of_steps,
            Direction::South => y += instruction.amount_of_steps,
            Direction::West => x -= instruction.amount_of_steps,
            Direction::North => y -= instruction.amount_of_steps,
            _ => panic!("Invalid direction"),
        }

        edges.push((x, y));
    }

    shoelace_area_of_polygon(&edges) + len / 2 + 1
}

fn two(input: impl Iterator<Item = String>) -> i64 {
    let instructions = input
        .map(|s| {
            let (_, c) = s.rsplit_once(' ').unwrap();
            c[2..c.len() - 1].to_string()
        })
        .map(Instruction::from_color);

    instructions_to_area(instructions)
}

fn one(input: impl Iterator<Item = String>) -> i64 {
    instructions_to_area(input.map(Instruction::from))
}

pub fn run(runner: &Runner) {
    let result = match runner.part {
        Part::One => one(get_non_empty_lines(&runner.path)),
        Part::Two => two(get_non_empty_lines(&runner.path)),
    };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

    #[test]
    fn test_one() {
        assert_eq!(one(TEST_INPUT.trim().lines().map(|s| s.to_string())), 62);
    }

    #[test]
    fn test_from_color() {
        assert_eq!(
            Instruction::from_color("70c710".to_string()),
            Instruction::from("R 461937")
        );
        assert_eq!(
            Instruction::from_color("caa171".to_string()),
            Instruction::from("D 829975")
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(
            two(TEST_INPUT.trim().lines().map(|s| s.to_string())),
            952408144115
        );
    }
}
