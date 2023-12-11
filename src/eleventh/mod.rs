use crate::{Part, Runner};

fn one() -> usize {
    0
}

fn two() -> usize {
    0
}

pub fn run(runner: &Runner) {
    let result = match runner.part {
        Part::One => one(),
        Part::Two => two(),
    };
    println!("result: {}", result)
}
