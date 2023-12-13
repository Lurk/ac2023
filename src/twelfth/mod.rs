use crate::{utils::get_non_empty_lines, Part, Runner};

fn parse_line(line: &str) -> Vec<String> {
    let (head, tail) = line.split_once(' ').unwrap();

    let sequence_of_damaged: Vec<usize> = tail
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let groups = head.chars().map(|c| match c {
        '?' => vec!["?"],
        '#' => vec!["#"],
        '.' => vec!["."],
        _ => panic!("invalid spring"),
    });
    let res = sequence_of_damaged.iter().fold(vec![], |mut acc, x| {
        acc.push("#".repeat(*x));
        acc
    });
    vec![res.join(".")]
}

fn one() -> usize {
    0
}

fn two() -> usize {
    0
}

pub fn run(runner: &Runner) {
    let mut line_length: usize = 0;
    let tiles: Vec<String> = get_non_empty_lines(&runner.path)
        .flat_map(|line| {
            line_length = line.trim().len();
            parse_line(line.trim())
        })
        .collect();

    let result = match runner.part {
        Part::One => one(),
        Part::Two => two(),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("???.### 1,1,3"), vec!["#.#.###"]);
        assert_eq!(
            parse_line(".??..??...?##. 1,1,3"),
            vec![
                "..#...#...###.",
                "..#..#....###.",
                ".#...#....###.",
                ".#....#...###.",
            ]
        );
    }
}
