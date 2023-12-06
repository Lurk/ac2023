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
