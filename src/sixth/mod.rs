use std::usize;

use crate::{utils::get_non_empty_lines, Runner};

enum LineType {
    Time(Vec<usize>),
    Distance(Vec<usize>),
}

impl TryFrom<String> for LineType {
    type Error = &'static str;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        if line.starts_with("Time:") {
            if let Some((_, tail)) = line.split_once(": ") {
                let time = tail
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect();
                return Ok(LineType::Time(time));
            }
        } else if line.starts_with("Distance:") {
            if let Some((_, tail)) = line.split_once(": ") {
                let distance = tail
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect();
                return Ok(LineType::Distance(distance));
            }
        }

        Err("Invalid line")
    }
}

fn get_time(time: usize, distance: usize) -> usize {
    let mut count = 0;
    for speed in 1..time {
        if (time - speed) * speed > distance {
            count += 1;
        } else if count > 0 {
            break;
        }
    }
    count
}

fn one(times: Vec<usize>, distances: Vec<usize>) {
    println!(
        "margin of error: {}",
        times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| get_time(*time, *distance))
            .product::<usize>()
    );
}

fn two(times: Vec<usize>, distances: Vec<usize>) {
    let time: usize = times
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let distance: usize = distances
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    println!("ways to beat the record: {}", get_time(time, distance));
}

pub fn run(runner: &Runner) {
    let mut times = vec![];
    let mut distances = vec![];
    for line in get_non_empty_lines(&runner.path) {
        match LineType::try_from(line) {
            Ok(LineType::Time(time)) => times = time,
            Ok(LineType::Distance(distance)) => distances = distance,
            Err(_) => println!("Invalid line"),
        }
    }

    match runner.part {
        crate::Part::One => one(times, distances),
        crate::Part::Two => two(times, distances),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_time() {
        assert_eq!(super::get_time(7, 9), 4);
        assert_eq!(super::get_time(15, 40), 8);
        assert_eq!(super::get_time(30, 200), 9);
    }
}
