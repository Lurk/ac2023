use crate::{
    utils::{get_lines, map::Map},
    Part, Runner,
};

fn one(mut map: Map<char>) -> usize {
    let columns: Vec<Vec<char>> = map
        .get_columns()
        .map(|c| {
            c.split(|s| *s == '#')
                .map(|s| {
                    let mut v = s.to_vec();
                    v.sort();
                    v.reverse();
                    v
                })
                .collect::<Vec<Vec<char>>>()
                .join(&'#')
        })
        .collect();

    map.replace_columns(columns);
    map.tiles
        .chunks(map.line_length)
        .rev()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .filter(|c| c == &&'O')
                .collect::<Vec<&char>>()
                .len()
                * (i + 1)
        })
        .sum()
}

fn lines_to_map(lines: impl Iterator<Item = String>) -> Map<char> {
    lines.map(|s| s.chars().collect::<Vec<char>>()).fold(
        Map {
            tiles: vec![],
            line_length: 0,
        },
        |mut acc, line| {
            if !line.is_empty() {
                acc.line_length = line.len();
                acc.tiles.extend(line);
            }
            acc
        },
    )
}

fn two(mut map: Map<char>) -> usize {
    0
}

pub fn run(runner: &Runner) {
    let map: Map<char> = lines_to_map(get_lines(&runner.path));

    let result = match runner.part {
        Part::One => one(map),
        Part::Two => two(map),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_one() {
        assert_eq!(one(lines_to_map(INPUT.lines().map(|s| s.to_string()))), 136);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(lines_to_map(INPUT.lines().map(|s| s.to_string()))), 0);
    }
}
