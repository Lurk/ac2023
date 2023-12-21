use crate::{
    utils::{get_non_empty_lines, map::Map},
    Part, Runner,
};

fn one(map: &Map<char>) -> usize {
    0
}

fn two(map: &Map<char>) -> usize {
    0
}

fn lines_to_map(lines: impl Iterator<Item = String>) -> Map<char> {
    lines.fold(
        Map {
            tiles: vec![],
            line_length: 0,
        },
        |mut map, line| {
            let row = line.trim().chars().collect::<Vec<char>>();
            map.line_length = row.len();
            map.tiles.extend(row);
            map
        },
    )
}

pub fn run(runner: &Runner) {
    let map = lines_to_map(get_non_empty_lines(&runner.path));

    let result = match runner.part {
        Part::One => one(&map),
        Part::Two => two(&map),
    };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    #[test]
    fn test_one() {
        let map = lines_to_map(TEST_INPUT.lines().map(String::from));

        assert_eq!(one(&map), 102);
    }
}
