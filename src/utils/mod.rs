pub mod direction;
pub mod map;

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

pub fn unique_permutations<T: Clone>(list: &[T]) -> Vec<Vec<T>> {
    if list.is_empty() {
        return vec![];
    }

    let mut permutations: Vec<Vec<T>> = Vec::new();
    for i in 0..list.len() - 1 {
        for j in i + 1..list.len() {
            permutations.push(vec![list[i].clone(), list[j].clone()]);
        }
    }

    permutations
}

pub fn combinations<T: Clone>(list: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return vec![];
    }

    if size == 1 {
        return list.iter().map(|x| vec![x.clone()]).collect();
    }

    let mut c: Vec<Vec<T>> = Vec::new();
    for i in 0..list.len() - size + 1 {
        let mut sub_combinations = combinations(&list[i + 1..], size - 1);
        for sub_combination in sub_combinations.iter_mut() {
            sub_combination.insert(0, list[i].clone());
        }
        c.append(&mut sub_combinations);
    }
    c
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

    #[test]
    fn test_unique_permutations() {
        assert_eq!(
            unique_permutations(&vec![1, 2, 3]),
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        );
    }

    #[test]
    fn test_combinations() {
        assert_eq!(
            combinations(&vec![1, 2, 3, 4], 3),
            vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4],]
        );
    }
}
