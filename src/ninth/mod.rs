use crate::{utils::get_non_empty_lines, Part, Runner};

fn get_steps(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut steps: Vec<Vec<i64>> = vec![input.clone()];
    while steps.last().unwrap().len() > 1 {
        steps.push(
            steps
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[1] - x[0])
                .skip(0)
                .collect(),
        );
    }
    steps
}

fn one(steps: Vec<Vec<i64>>) -> i64 {
    steps
        .iter()
        .rev()
        .map(|x| x.last().unwrap())
        .cloned()
        .reduce(|acc, x| acc + x)
        .unwrap()
}

fn two(steps: Vec<Vec<i64>>) -> i64 {
    steps
        .iter()
        .rev()
        .map(|x| x.first().unwrap())
        .cloned()
        .reduce(|acc, x| x - acc)
        .unwrap()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ').map(|x| x.parse().unwrap()).collect()
}

pub fn run(runner: &Runner) {
    let iter = get_non_empty_lines(&runner.path)
        .map(|line| parse_line(&line))
        .map(get_steps);

    let result: i64 = match runner.part {
        Part::One => iter.map(one).sum(),
        Part::Two => iter.map(two).sum(),
    };

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one(get_steps(vec![1, 2, 3])), 4);
        assert_eq!(one(get_steps(vec![0, 3, 6, 9, 12, 15])), 18);
        assert_eq!(one(get_steps(vec![1, 3, 6, 10, 15, 21])), 28);
        assert_eq!(one(get_steps(vec![10, 13, 16, 21, 30, 45])), 68);
        assert_eq!(
            one(get_steps(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])),
            11
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(two(get_steps(vec![1, 2, 3])), 0);
        assert_eq!(two(get_steps(vec![0, 3, 6, 9, 12, 15])), -3);
        assert_eq!(two(get_steps(vec![1, 3, 6, 10, 15, 21])), 0);
        assert_eq!(two(get_steps(vec![10, 13, 16, 21, 30, 45])), 5);
        assert_eq!(two(get_steps(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8])), -2);
    }
}
