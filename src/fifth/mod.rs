use std::usize;

use rayon::prelude::*;

use crate::{utils::get_lines, Runner};

#[derive(Debug)]
struct Range {
    source: usize,
    destination: usize,
    length: usize,
}

impl Range {
    fn new(source: usize, destination: usize, length: usize) -> Self {
        Self {
            source,
            destination,
            length,
        }
    }
}

enum LineType {
    Seeds(Vec<usize>),
    MapType,
    MapVale(Range),
    Empty,
}

impl TryFrom<String> for LineType {
    type Error = &'static str;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        if line.starts_with("seeds:") {
            if let Some((_, tail)) = line.split_once(": ") {
                let seeds = tail.split(' ').map(|x| x.parse().unwrap()).collect();
                return Ok(LineType::Seeds(seeds));
            }
        } else if line.ends_with("map:") {
            if let Some((head, _)) = line.split_once(' ') {
                if head.split_once("-to-").is_some() {
                    return Ok(LineType::MapType);
                }
            }
        } else if line.is_empty() {
            return Ok(LineType::Empty);
        } else if line.chars().nth(0).unwrap().is_ascii_digit() {
            if let Some((destination, tail)) = line.split_once(' ') {
                if let Some((source, length)) = tail.split_once(' ') {
                    return Ok(LineType::MapVale(Range::new(
                        source.parse().unwrap(),
                        destination.parse().unwrap(),
                        length.parse().unwrap(),
                    )));
                }
            }
        }

        Err("Invalid line")
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    fn get_destination(&self, source: usize) -> usize {
        let mut destination = source;
        for range in &self.ranges {
            if range.source <= source && source < range.source + range.length {
                destination = range.destination + (source - range.source);
                break;
            }
        }
        destination
    }
}

fn one(maps: Vec<Map>, seeds: Vec<usize>) -> usize {
    seeds
        .par_iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.get_destination(acc)))
        .min()
        .unwrap()
}

fn two(maps: Vec<Map>, seeds: Vec<usize>) -> usize {
    seeds
        .chunks(2)
        .map(|x| {
            (x[0]..x[0] + x[1])
                .into_par_iter()
                .map(|source| {
                    maps.iter()
                        .fold(source, |acc, map| map.get_destination(acc))
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

pub fn run(runner: &Runner) {
    let mut seeds: Vec<usize> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    let mut map: Option<Map> = None;

    for line in get_lines(&runner.path) {
        let val = LineType::try_from(line).unwrap();

        match val {
            LineType::Seeds(s) => {
                seeds = s;
            }
            LineType::MapType => {
                map = Some(Map::new());
            }
            LineType::MapVale(range) => {
                map.as_mut().unwrap().add_range(range);
            }
            LineType::Empty => {
                if map.is_some() {
                    maps.push(map.unwrap());
                    map = None;
                }
            }
        }
    }

    if let Some(map) = map {
        maps.push(map);
    }

    let location = match runner.part {
        crate::Part::One => one(maps, seeds),
        crate::Part::Two => two(maps, seeds),
    };

    println!("Location: {:?}", location);
}

#[cfg(test)]
mod test {
    use crate::fifth::LineType;

    use super::{two, Map};

    #[test]
    fn min_location_part_two() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let mut maps: Vec<Map> = Vec::new();
        let mut map: Option<Map> = None;
        let mut seeds: Vec<usize> = Vec::new();
        for line in input.lines() {
            match LineType::try_from(line.to_string()).unwrap() {
                LineType::Seeds(s) => {
                    seeds = s;
                }
                LineType::MapType => {
                    map = Some(Map::new());
                }
                LineType::MapVale(range) => {
                    map.as_mut().unwrap().add_range(range);
                }
                LineType::Empty => {
                    if map.is_some() {
                        maps.push(map.unwrap());
                        map = None;
                    }
                }
            }
        }
        if map.is_some() {
            maps.push(map.unwrap());
        }

        assert_eq!(46, two(maps, seeds));
    }

    #[test]
    fn get_destination() {
        let mut map = Map::new();

        map.add_range(super::Range::new(0, 15, 10));
        map.add_range(super::Range::new(37, 52, 2));
        map.add_range(super::Range::new(39, 0, 15));

        assert_eq!(15, map.get_destination(0));
        assert_eq!(52, map.get_destination(37));
        assert_eq!(0, map.get_destination(39));
    }
}
