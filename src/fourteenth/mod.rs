use crate::{
    utils::{cycle::CycleDetector, get_lines, map::Map},
    Part, Runner,
};

fn north(map: &mut Map<char>) {
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
}

fn south(map: &mut Map<char>) {
    let columns: Vec<Vec<char>> = map
        .get_columns()
        .map(|c| {
            c.split(|s| *s == '#')
                .map(|s| {
                    let mut v = s.to_vec();
                    v.sort();
                    v
                })
                .collect::<Vec<Vec<char>>>()
                .join(&'#')
        })
        .collect();

    map.replace_columns(columns);
}

fn west(map: &mut Map<char>) {
    let rows: Vec<Vec<char>> = map
        .get_rows()
        .map(|r| {
            r.split(|s| *s == '#')
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

    map.replace_rows(rows);
}

fn east(map: &mut Map<char>) {
    let rows: Vec<Vec<char>> = map
        .get_rows()
        .map(|r| {
            r.split(|s| *s == '#')
                .map(|s| {
                    let mut v = s.to_vec();
                    v.sort();
                    v
                })
                .collect::<Vec<Vec<char>>>()
                .join(&'#')
        })
        .collect();

    map.replace_rows(rows);
}

fn get_weigth(map: &Map<char>) -> usize {
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

fn one(mut map: Map<char>) -> usize {
    north(&mut map);

    get_weigth(&map)
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
    let mut detector = CycleDetector::new();
    let n = 1_000_000_000;
    for _ in 0..n {
        north(&mut map);
        west(&mut map);
        south(&mut map);
        east(&mut map);
        let weigth = get_weigth(&map);
        if let Some(i) = detector.push(weigth) {
            let slice = detector.get_slice(i, detector.len());
            let position = (n - i) % slice.len() - 1;
            return slice[position];
        }
    }
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
        assert_eq!(two(lines_to_map(INPUT.lines().map(|s| s.to_string()))), 64);
    }
}
