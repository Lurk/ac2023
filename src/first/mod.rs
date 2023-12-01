use std::{fs::read_to_string, path::PathBuf, u64, usize};

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn get_from_slice(slice: &str) -> Option<char> {
    for (n, number) in NUMBERS.iter().enumerate() {
        if slice.starts_with(number) {
            return Some(CHARS[n]);
        }
    }
    None
}

pub fn process(line: &str) -> usize {
    let mut str = String::new();
    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            str.push(char);
            break;
        }

        if let Some(number) = get_from_slice(&line[index..]) {
            str.push(number);
            break;
        }
    }

    for (index, char) in line.chars().rev().enumerate() {
        if char.is_numeric() {
            str.push(char);
            break;
        }

        if let Some(number) = get_from_slice(&line[line.len() - (index + 1)..]) {
            str.push(number);
            break;
        }
    }

    if str.len() == 0 {
        return 0;
    }
    str.parse::<usize>().unwrap()
}

pub fn run(path: PathBuf) {
    let input = read_to_string(path).unwrap();
    let mut lines = input.lines();
    let mut total: u64 = 0;
    while let Some(line) = lines.next() {
        total += process(line) as u64;
    }
    println!("{}", total);
}
