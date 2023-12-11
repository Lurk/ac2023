pub mod direction;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn get_non_empty_lines(path: &PathBuf) -> impl Iterator<Item = std::string::String> {
    get_lines(path).filter(|x| !x.is_empty())
}

pub fn get_lines(path: &PathBuf) -> impl Iterator<Item = std::string::String> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    reader.lines().map(|x| x.unwrap())
}

pub fn euclidic_gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        euclidic_gcd(b, a % b)
    }
}

pub fn euclidic_lcm(a: usize, b: usize) -> usize {
    a * b / euclidic_gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidic_gcd() {
        assert_eq!(euclidic_gcd(10, 9), 1);
        assert_eq!(euclidic_gcd(10, 10), 10);
        assert_eq!(euclidic_gcd(30, 33), 3);
        assert_eq!(euclidic_gcd(18, 48), 6);
        assert_eq!(euclidic_gcd(48, 18), 6);
    }

    #[test]
    fn test_euclidic_lcm() {
        assert_eq!(euclidic_lcm(10, 9), 90);
        assert_eq!(euclidic_lcm(10, 10), 10);
        assert_eq!(euclidic_lcm(30, 33), 330);
        assert_eq!(euclidic_lcm(18, 48), 144);
        assert_eq!(euclidic_lcm(48, 18), 144);
    }
}
