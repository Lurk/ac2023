use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    usize,
};

use crate::{
    utils::{direction::Direction, get_non_empty_lines, map::Map},
    Part, Runner,
};

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Vertex {
    index: usize,
    direction: Direction,
    amount_of_steps_in_same_direction: usize,
}

#[derive(Debug)]
struct Graph {
    vertices: Vec<Arc<Vertex>>,
    edges: HashMap<Arc<Vertex>, HashSet<(usize, Arc<Vertex>)>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: vec![],
            edges: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: Arc<Vertex>) -> bool {
        if self.edges.contains_key(&vertex) {
            return false;
        }
        self.vertices.push(vertex.clone());
        self.edges.insert(vertex, HashSet::new());
        true
    }

    fn add_edge(&mut self, from: Arc<Vertex>, to: Arc<Vertex>, length: usize) {
        self.edges.entry(from).or_default().insert((length, to));
    }

    fn get_edges(&self, vertex: Arc<Vertex>) -> Option<&HashSet<(usize, Arc<Vertex>)>> {
        self.edges.get(&vertex)
    }

    fn get_vertex(&self, index: usize) -> Option<&Arc<Vertex>> {
        self.vertices.get(index)
    }
}

fn build_graph(map: &Map<usize>) -> Graph {
    let mut graph = Graph::new();
    let mut queue: Vec<Arc<Vertex>> = vec![];
    let start = Arc::new(Vertex {
        index: 0,
        direction: Direction::Center,
        amount_of_steps_in_same_direction: 1,
    });

    graph.add_vertex(start.clone());
    queue.push(start.clone());

    while let Some(current) = queue.pop() {
        for d in &DIRECTIONS {
            if d == &current.direction.opposite() {
                continue;
            }

            if current.amount_of_steps_in_same_direction >= 3 && d == &current.direction {
                continue;
            }

            if let Some(i) = map.move_from(current.index, d) {
                let next = Arc::new(Vertex {
                    index: i,
                    direction: d.clone(),
                    amount_of_steps_in_same_direction: if d == &current.direction {
                        current.amount_of_steps_in_same_direction + 1
                    } else {
                        1
                    },
                });

                if graph.add_vertex(next.clone()) {
                    queue.push(next.clone());
                }
                graph.add_edge(current.clone(), next.clone(), map.tiles[i]);
            }
        }
    }
    graph
}

fn dijkstra(graph: &Graph, source: Arc<Vertex>) -> HashMap<Arc<Vertex>, usize> {
    let mut distances: HashMap<Arc<Vertex>, usize> = HashMap::new();
    let mut previous: HashMap<Arc<Vertex>, Option<Arc<Vertex>>> = HashMap::new();
    let mut queue: HashMap<Arc<Vertex>, usize> = HashMap::new();

    for v in &graph.vertices {
        distances.insert(v.clone(), usize::MAX);
        previous.insert(v.clone(), None);
    }

    queue.insert(source.clone(), 0);
    distances.insert(source.clone(), 0);

    while !queue.is_empty() {
        let (u, _) = queue.iter().min_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
        let u = u.clone();
        queue.remove(&u);

        if let Some(neighbors) = graph.get_edges(u.clone()) {
            for (distance, v) in neighbors {
                let alt = distances.get(&u).unwrap() + distance;
                if alt < *distances.get(v).unwrap() {
                    queue.insert(v.clone(), alt);
                    distances.entry(v.clone()).and_modify(|v| *v = alt);
                    previous
                        .entry(v.clone())
                        .and_modify(|v| *v = Some(u.clone()));
                }
            }
        }
    }
    distances
}

fn one(map: &Map<usize>) -> usize {
    println!("one");
    let graph = build_graph(map);
    println!("graph built");
    let distances = dijkstra(&graph, graph.get_vertex(0).unwrap().clone());
    println!("dijkstra done");
    *distances
        .iter()
        .filter(|(k, _)| k.index == map.tiles.len() - 1)
        .map(|(_, v)| v)
        .min()
        .unwrap()
}

fn two(_map: &Map<usize>) -> usize {
    0
}

fn lines_to_map(lines: impl Iterator<Item = String>) -> Map<usize> {
    lines.fold(
        Map {
            tiles: vec![],
            line_length: 0,
        },
        |mut map, line| {
            let row = line
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            map.line_length = row.len();
            map.tiles.extend(row);
            map
        },
    )
}

pub fn run(runner: &Runner) {
    let map = lines_to_map(get_non_empty_lines(&runner.path));

    let result = match runner.part {
        Part::One => one(&map),
        Part::Two => two(&map),
    };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    // 241343
    // 321545
    // 325524
    // 344658
    // 454665
    // 143859

    // 0    3   5   8   19   x
    // 3    5   6   11  15   x
    // 6    7   11  16  17   x
    // 9    11  15  21   x   x
    // 20   16  19   x   x   x
    // 21   20   x   x   x   x
    #[test]
    fn test_one() {
        let map = lines_to_map(TEST_INPUT.lines().map(String::from));
        assert_eq!(one(&map), 102);
    }
}
