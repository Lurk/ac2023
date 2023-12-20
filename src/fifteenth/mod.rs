use std::sync::Arc;

use crate::{utils::get_non_empty_lines, Part, Runner};

pub fn one(instructions: impl Iterator<Item = String>) -> usize {
    instructions.map(|s| hash(s.as_str())).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: Arc<str>,
    power: Option<usize>,
}

impl TryFrom<&str> for Lens {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some((label, power)) = s.split_once('=') {
            let power = power.parse::<usize>().ok();
            return Ok(Self {
                label: Arc::from(label),
                power,
            });
        }
        if let Some((label, _)) = s.split_once('-') {
            return Ok(Self {
                label: Arc::from(label),
                power: None,
            });
        }
        Err(format!("invalid lens {}", s))
    }
}

fn assign_lenses(instructions: impl Iterator<Item = String>) -> Vec<Vec<Lens>> {
    instructions
        .map(|s| Lens::try_from(s.as_str()).unwrap())
        .fold(vec![vec![]; 256], |mut acc: Vec<Vec<Lens>>, lens: Lens| {
            let box_id = hash(lens.label.as_ref());
            if let Some(i) = acc[box_id].iter().position(|l| l.label == lens.label) {
                let box_content = acc.get_mut(box_id).unwrap();
                if lens.power.is_none() {
                    box_content.remove(i);
                } else {
                    box_content[i] = lens;
                }
            } else if lens.power.is_some() {
                acc.get_mut(box_id).unwrap().push(lens);
            }

            acc
        })
}

pub fn two(instructions: impl Iterator<Item = String>) -> usize {
    let boxes: Vec<Vec<Lens>> = assign_lenses(instructions);

    boxes
        .iter()
        .enumerate()
        .flat_map(|(b_i, box_content)| {
            box_content
                .iter()
                .enumerate()
                .map(|(l_i, lens)| (b_i + 1) * (l_i + 1) * lens.power.unwrap_or(0))
                .collect::<Vec<usize>>()
        })
        .sum()
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

    #[test]
    fn lenses() {
        let vals = vec![
            "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
        ];
        assert_eq!(super::two(vals.into_iter().map(|s| s.to_string())), 145);
    }
}
