use std::{collections::HashMap, sync::Arc};

use crate::{
    utils::{euclidic_lcm, get_lines},
    Runner,
};

enum LineType {
    Direction(Vec<Direction>),
    Node(Arc<str>, Arc<str>, Arc<str>),
    Empty,
}

enum Direction {
    Left,
    Right,
}

pub struct Node {
    left: Arc<str>,
    right: Arc<str>,
}

pub struct Map {
    steps: Vec<Direction>,
    graph: HashMap<Arc<str>, Node>,
}

impl Map {
    fn new() -> Self {
        Self {
            steps: Vec::new(),
            graph: HashMap::new(),
        }
    }

    fn replace_steps(&mut self, steps: Vec<Direction>) {
        self.steps = steps;
    }

    pub fn add_node(&mut self, node: (Arc<str>, Arc<str>, Arc<str>)) {
        self.graph.insert(
            node.0,
            Node {
                left: node.1,
                right: node.2,
            },
        );
    }

    pub fn steps_from_to(&self, from: Arc<str>, to: Arc<str>) -> usize {
        let mut step = 0;
        let mut count = 0;
        let mut current = from;
        while current != to {
            match self.steps[step] {
                Direction::Left => current = self.graph[&current].left.clone(),
                Direction::Right => current = self.graph[&current].right.clone(),
            }
            step = (step + 1) % self.steps.len();
            count += 1;
        }
        count
    }

    pub fn steps_from_to_ends_with(&self, from: char, to: char) -> usize {
        self.graph
            .keys()
            .filter(|key| key.ends_with(from))
            .cloned()
            .map(|k| {
                let mut key = &k;
                let mut count = 0;
                loop {
                    for step in &self.steps {
                        key = match step {
                            Direction::Left => &self.graph[key].left,
                            Direction::Right => &self.graph[key].right,
                        };
                        count += 1;
                        if key.ends_with(to) {
                            return count;
                        }
                    }
                }
            })
            .reduce(euclidic_lcm)
            .unwrap()
    }
}

pub fn one(map: Map) -> usize {
    map.steps_from_to(Arc::from("AAA"), Arc::from("ZZZ"))
}

pub fn two(map: Map) -> usize {
    map.steps_from_to_ends_with('A', 'Z')
}

fn parse_line(str: &str, line_number: usize) -> Result<LineType, &'static str> {
    if str.is_empty() {
        return Ok(LineType::Empty);
    }

    if line_number == 0 {
        return Ok(LineType::Direction(
            str.chars()
                .map(|c| match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Invalid direction"),
                })
                .collect(),
        ));
    }
    if let Some((head, tail)) = str.split_once(" = ") {
        if let Some((left, right)) = tail.trim_matches(|c| c == '(' || c == ')').split_once(", ") {
            return Ok(LineType::Node(
                Arc::from(head),
                Arc::from(left),
                Arc::from(right),
            ));
        }
    }

    Err("Invalid line")
}

pub fn run(runner: &Runner) {
    let mut map = Map::new();

    for (index, line) in get_lines(&runner.path).enumerate() {
        let line = parse_line(&line, index).unwrap();
        match line {
            LineType::Direction(directions) => map.replace_steps(directions),
            LineType::Node(name, left, right) => map.add_node((name, left, right)),
            LineType::Empty => (),
        }
    }

    let result = match runner.part {
        crate::Part::One => one(map),
        crate::Part::Two => two(map),
    };

    println!("Result: {}", result);
}

#[cfg(test)]
mod test {
    #[test]
    fn part_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let mut map = super::Map::new();
        for (index, line) in input.lines().enumerate() {
            let line = super::parse_line(line, index).unwrap();
            match line {
                super::LineType::Direction(directions) => map.replace_steps(directions),
                super::LineType::Node(name, left, right) => map.add_node((name, left, right)),
                super::LineType::Empty => (),
            }
        }

        assert_eq!(super::two(map), 6);
    }
}
