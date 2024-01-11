use crate::{Part, Runner};

fn one() -> usize {
    0
}

pub fn run(runner: &Runner) {
    match runner.part {
        Part::One => {
            // let mut modules = parse_input(&runner.get_input(0));
            // runner.print_answer(1, one(&mut modules));
        }
        Part::Two => {
            // let mut modules = parse_input(&runner.get_input(0));
            // runner.print_answer(2, two(&mut modules));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

    #[test]
    fn test_one() {
        // let mut modules = parse_input(INPUT);
        // assert_eq!(one(&mut modules), 0);
    }

    #[test]
    fn test_two() {
        // let mut modules = parse_input(INPUT);
        // assert_eq!(two(&mut modules), 0);
    }
}
