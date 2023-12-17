use std::{
    fmt::{Display, Formatter},
    usize,
};

use crate::{utils::get_non_empty_lines, Part, Runner};

struct Line {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.springs
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(""),
            self.groups
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Line {
    fn get_amount_if_combinations(&self) -> usize {
        self.check_partial("", 0)
    }

    fn check_partial(&self, s: &str, g_i: usize) -> usize {
        let s_i = s.len();
        if s_i == 0 {
            if self.springs[0] == '?' {
                return self.check_partial(".", g_i) + self.check_partial("#", g_i);
            } else {
                return self.check_partial(format!("{}", self.springs[0]).as_str(), g_i);
            }
        } else if s_i < self.springs.len() && g_i < self.groups.len() {
            let rest = self.groups[g_i + 1..].iter().sum::<usize>() + self.groups.len() - g_i - 1;
            let add = if s_i + 1 == self.springs.len() { 1 } else { 0 };

            if self.springs[s_i] == '?' {
                if s.ends_with("#".repeat(self.groups[g_i]).as_str()) {
                    return self.check_partial(&format!("{}.", s), g_i + 1);
                } else if s.ends_with('#') {
                    return self.check_partial(&format!("{}#", s), g_i + add);
                } else if s.ends_with('.') {
                    return self.check_partial(&format!("{}.", s), g_i)
                        + self.check_partial(&format!("{}#", s), g_i + add);
                }
            } else if self.springs[s_i] == '.' {
                if s.ends_with("#".repeat(self.groups[g_i]).as_str()) {
                    return self.check_partial(&format!("{}.", s), g_i + 1);
                } else if s.ends_with('.') && s_i + rest < self.springs.len() - 1 {
                    return self.check_partial(&format!("{}.", s), g_i + add);
                }
            } else if self.springs[s_i] == '#'
                && !s.ends_with("#".repeat(self.groups[g_i]).as_str())
            {
                return self.check_partial(&format!("{}#", s), g_i + add);
            }
        } else if g_i == self.groups.len()
            && s_i <= self.springs.len()
            && !self.springs[s_i..].iter().any(|x| *x == '#')
            && (s.ends_with(format!(".{}", "#".repeat(self.groups[g_i - 1])).as_str())
                || s.ends_with('.'))
        {
            return 1;
        }
        0
    }
}

fn parse_line(line: &str, fold: bool) -> Line {
    let (head, tail) = line.split_once(' ').unwrap();

    let n = if fold { 5 } else { 1 };
    let groups: Vec<usize> = vec![tail; n]
        .join(",")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let springs = vec![head; n]
        .join("?")
        .trim_matches('.')
        .chars()
        .fold(vec![], |mut acc, c| {
            // remove duplicate dots
            if c == '.' && acc.last() == Some(&'.') {
                acc
            } else {
                acc.push(c);
                acc
            }
        });

    Line { springs, groups }
}

fn one(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|line| parse_line(line.trim(), false))
        .fold(0, |acc, line| acc + line.get_amount_if_combinations())
}

fn two(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|line| parse_line(line.trim(), true))
        .fold(0, |acc, line| acc + line.get_amount_if_combinations())
}

pub fn run(runner: &Runner) {
    let lines = get_non_empty_lines(&runner.path);

    let result = match runner.part {
        Part::One => one(lines),
        Part::Two => two(lines),
    };
    println!("result: {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("???.### 1,1,3", false).get_amount_if_combinations(),
            1
        );
        assert_eq!(
            parse_line("..??..??...?##.. 1,1,3", false).get_amount_if_combinations(),
            4
        );
        assert_eq!(
            parse_line("?#?#?#?#?#?#?#? 1,3,1,6", false).get_amount_if_combinations(),
            1
        );
        assert_eq!(
            parse_line("????.######..#####. 1,6,5", false).get_amount_if_combinations(),
            4
        );

        assert_eq!(
            parse_line("?###???????? 3,2,1", false).get_amount_if_combinations(),
            10
        );

        assert_eq!(
            parse_line("????.#...#... 4,1,1", false).get_amount_if_combinations(),
            1
        );

        assert_eq!(
            parse_line("#.#?. 1,1", false).get_amount_if_combinations(),
            1
        );
        assert_eq!(
            parse_line("??#???##?? 1,3", false).get_amount_if_combinations(),
            2
        );
        assert_eq!(parse_line("#??# 2", false).get_amount_if_combinations(), 0);
    }

    #[test]
    fn test_one() {
        let lines = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(one(lines.lines().map(|x| x.to_string())), 21);
    }
}
