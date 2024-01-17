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

fn find_reflection(map: &Map<char>, skip: &Option<Type>) -> Type {
    let row = map.get_rows().next().unwrap();
    for i in 1..map.line_length {
        if skip == &Some(Type::Horizontal(i)) {
            continue;
        }
        if is_reflection_at_index(&row, i) && map.get_rows().all(|r| is_reflection_at_index(&r, i))
        {
            return Type::Horizontal(i);
        }
    }
    let column = map.get_columns().next().unwrap();
    for i in 1..map.get_rows_count() {
        if skip == &Some(Type::Vertical(i)) {
            continue;
        }
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
    return (head.len() > tail.len()
        && head.ends_with(tail.iter().cloned().rev().collect::<Vec<_>>().as_slice()))
        || (tail.len() > head.len()
            && tail.starts_with(head.iter().cloned().rev().collect::<Vec<_>>().as_slice()));
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
        .map(|m| find_reflection(&m, &None))
        .map(|t| match t {
            Type::Horizontal(i) => i,
            Type::Vertical(i) => i * 100,
            Type::None => 0,
        })
        .sum()
}

fn fix_the_smudge(map: &Map<char>, skip: Option<Type>) -> Type {
    for j in 0..map.tiles.len() {
        let mut updated_map = map.clone();
        if updated_map.tiles[j] == '#' {
            updated_map.tiles[j] = '.';
        } else {
            updated_map.tiles[j] = '#';
        }

        match find_reflection(&updated_map, &skip) {
            Type::Horizontal(i) => return Type::Horizontal(i),
            Type::Vertical(i) => return Type::Vertical(i),
            Type::None => continue,
        }
    }
    Type::None
}

fn two(lines: impl Iterator<Item = String>) -> usize {
    get_maps(lines)
        .into_iter()
        .filter(|m| !m.is_empty())
        .map(|m| fix_the_smudge(&m, Some(find_reflection(&m, &None))))
        .map(|t| match t {
            Type::Horizontal(i) => i,
            Type::Vertical(i) => i * 100,
            Type::None => 0,
        })
        .sum()
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

    const INPUT: &str = r#"#.##..##.
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

"#;

    #[test]
    fn test_parse() {
        let maps = get_maps(INPUT.lines().map(|l| l.to_string()));
        assert_eq!(maps.len(), 3);

        assert_eq!(find_reflection(&maps[0], &None), Type::Horizontal(5));
        assert_eq!(find_reflection(&maps[1], &None), Type::Vertical(4));
    }

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT.lines().map(|l| l.to_string())), 405);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT.lines().map(|l| l.to_string())), 400);
    }
}
