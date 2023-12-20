use std::usize;

use crate::{utils::get_non_empty_lines, Part, Runner};

pub fn one(instructions: impl Iterator<Item = String>) -> usize {
    instructions.map(|s| hash(s.as_str())).sum()
}

pub fn two(instructions: impl Iterator<Item = String>) -> usize {
    0
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |acc, c| ((acc + *c as usize) * 17) % 256)
}

pub fn run(runner: &Runner) {
    let instructions = get_non_empty_lines(&runner.path)
        .flat_map(|s| s.split(',').map(|s| s.to_string()).collect::<Vec<String>>());

    let result = match runner.part {
        Part::One => one(instructions),
        Part::Two => two(instructions),
    };

    println!("result {}", result);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hash() {
        assert_eq!(super::hash("HASH"), 52);
        assert_eq!(super::hash("rn=1"), 30);
        assert_eq!(super::hash("cm-"), 253);
        assert_eq!(super::hash("qp=3"), 97);
        assert_eq!(super::hash("cm=2"), 47);
        assert_eq!(super::hash("qp-"), 14);
        assert_eq!(super::hash("pc=4"), 180);
        assert_eq!(super::hash("ot=9"), 9);
    }
}
