use crate::{
    utils::{get_lines, map::Map},
    Part, Runner,
};

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Vertical(usize),
    Horizontal(usize),
    None,
}

fn find_reflection(map: &Map<char>) -> Type {
    let row = map.get_rows().nth(0).unwrap();
    for i in 1..map.line_length {
        if is_reflection_at_index(&row, i) && map.get_rows().all(|r| is_reflection_at_index(&r, i))
        {
            return Type::Horizontal(i);
        }
    }
    let column = map.get_columns().nth(0).unwrap();
    for i in 1..map.get_columns_count() {
        if is_reflection_at_index(&column, i)
            && map.get_columns().all(|c| is_reflection_at_index(&c, i))
        {
            return Type::Vertical(i);
        }
    }
    Type::None
}

fn is_reflection_at_index(slice: &[char], i: usize) -> bool {
    let (head, tail) = slice.split_at(i);
    if head.len() > tail.len()
        && head.ends_with(tail.iter().cloned().rev().collect::<Vec<_>>().as_slice())
    {
        return true;
    } else if tail.len() > head.len()
        && tail.starts_with(head.iter().cloned().rev().collect::<Vec<_>>().as_slice())
    {
        return true;
    }

    return false;
}

fn get_maps(lines: impl Iterator<Item = String>) -> Vec<Map<char>> {
    lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .fold(vec![], |mut acc, line| {
            if line.is_empty() {
                acc.push(Map {
                    tiles: vec![],
                    line_length: 0,
                });
            } else if acc.is_empty() {
                acc.push(Map {
                    line_length: line.len(),
                    tiles: line,
                });
            } else {
                let map = acc.last_mut().unwrap();
                map.line_length = line.len();
                map.tiles.extend(line);
            }
            acc
        })
}

fn one(lines: impl Iterator<Item = String>) -> usize {
    get_maps(lines)
        .into_iter()
        .filter(|m| !m.is_empty())
        .map(|m| find_reflection(&m))
        .map(|t| match t {
            Type::Horizontal(i) => i,
            Type::Vertical(i) => i * 100,
            Type::None => 0,
        })
        .sum()
}

fn two(lines: impl Iterator<Item = String>) -> usize {
    0
}

pub fn run(runner: &Runner) {
    let lines = get_lines(&runner.path);

    let result = match runner.part {
        Part::One => one(lines),
        Part::Two => two(lines),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

"#
        .lines()
        .map(|l| l.to_string());

        let maps = get_maps(input);
        assert_eq!(maps.len(), 3);

        assert_eq!(find_reflection(&maps[0]), Type::Horizontal(5));
        assert_eq!(find_reflection(&maps[1]), Type::Vertical(4));
    }

    #[test]
    fn test_one() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

"#
        .lines()
        .map(|l| l.to_string());
        assert_eq!(one(input), 405);
    }

    #[test]
    fn test_two() {
        let input = r#""#.lines().map(|l| l.to_string());
        assert_eq!(two(input), 0);
    }
}
