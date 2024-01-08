use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use crate::{utils::get_non_empty_lines, Runner};

#[derive(Debug, Clone, Eq, PartialEq)]
enum SignalType {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Signal {
    origin: usize,
    destination: usize,
    value: SignalType,
}

trait Module {
    fn id(&self) -> usize;
    fn receive(&mut self, signal: &Signal) -> Vec<Signal>;
    fn destinations(&self) -> &[usize];
}

#[derive(Debug, Clone)]
struct Broadcast {
    id: usize,
    destinations: Vec<usize>,
}

impl Module for Broadcast {
    fn id(&self) -> usize {
        self.id
    }
    fn receive(&mut self, signal: &Signal) -> Vec<Signal> {
        self.destinations
            .iter()
            .map(|destination| Signal {
                origin: signal.destination,
                destination: *destination,
                value: signal.value.clone(),
            })
            .collect()
    }
    fn destinations(&self) -> &[usize] {
        &self.destinations
    }
}

#[derive(Debug, Clone)]
struct Conjuction {
    id: usize,
    inputs: HashMap<usize, SignalType>,
    destinations: Vec<usize>,
}

impl Module for Conjuction {
    fn id(&self) -> usize {
        self.id
    }
    fn receive(&mut self, signal: &Signal) -> Vec<Signal> {
        self.inputs.insert(signal.origin, signal.value.clone());
        let signal_type = if self.inputs.values().all(|value| value == &SignalType::High) {
            SignalType::Low
        } else {
            SignalType::High
        };

        self.destinations
            .iter()
            .map(|destination| Signal {
                origin: self.id(),
                destination: *destination,
                value: signal_type.clone(),
            })
            .collect()
    }
    fn destinations(&self) -> &[usize] {
        &self.destinations
    }
}

impl Conjuction {
    fn add_input(&mut self, input: usize) {
        self.inputs.insert(input, SignalType::Low);
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    id: usize,
    state: SignalType,
    destinations: Vec<usize>,
}

impl Module for FlipFlop {
    fn id(&self) -> usize {
        self.id
    }
    fn receive(&mut self, signal: &Signal) -> Vec<Signal> {
        if signal.value == SignalType::Low {
            self.state = match self.state {
                SignalType::High => SignalType::Low,
                SignalType::Low => SignalType::High,
            };
            return self
                .destinations
                .iter()
                .map(|destination| Signal {
                    origin: signal.destination,
                    destination: *destination,
                    value: self.state.clone(),
                })
                .collect();
        }
        vec![]
    }
    fn destinations(&self) -> &[usize] {
        &self.destinations
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
    Broadcast(Broadcast),
    Empty,
}

impl Module for ModuleType {
    fn id(&self) -> usize {
        match self {
            ModuleType::FlipFlop(module) => module.id(),
            ModuleType::Conjuction(module) => module.id(),
            ModuleType::Broadcast(module) => module.id(),
            ModuleType::Empty => usize::MAX,
        }
    }
    fn receive(&mut self, signal: &Signal) -> Vec<Signal> {
        match self {
            ModuleType::FlipFlop(module) => module.receive(signal),
            ModuleType::Conjuction(module) => module.receive(signal),
            ModuleType::Broadcast(module) => module.receive(signal),
            ModuleType::Empty => vec![],
        }
    }
    fn destinations(&self) -> &[usize] {
        match self {
            ModuleType::FlipFlop(module) => module.destinations(),
            ModuleType::Conjuction(module) => module.destinations(),
            ModuleType::Broadcast(module) => module.destinations(),
            ModuleType::Empty => &[],
        }
    }
}

impl From<Broadcast> for ModuleType {
    fn from(value: Broadcast) -> Self {
        ModuleType::Broadcast(value)
    }
}

impl From<Conjuction> for ModuleType {
    fn from(value: Conjuction) -> Self {
        ModuleType::Conjuction(value)
    }
}

impl From<FlipFlop> for ModuleType {
    fn from(value: FlipFlop) -> Self {
        ModuleType::FlipFlop(value)
    }
}

enum ModuleTypeBuilder {
    Broadcast,
    Conjuction,
    FlipFlop,
}

struct ModuleBuilder {
    id: String,
    destinations: Vec<String>,
    t: ModuleTypeBuilder,
}

impl From<String> for ModuleBuilder {
    fn from(value: String) -> Self {
        let (id, destinations) = value
            .split_once(" -> ")
            .expect("should have an id and destionation");
        let destinations = destinations.split(", ").map(String::from).collect();
        let (id, t) = if id == "broadcaster" {
            ("broadcaster", ModuleTypeBuilder::Broadcast)
        } else if let Some(id) = id.strip_prefix('&') {
            (id, ModuleTypeBuilder::Conjuction)
        } else if let Some(id) = id.strip_prefix('%') {
            (id, ModuleTypeBuilder::FlipFlop)
        } else {
            panic!("unknown module type {}", id)
        };
        Self {
            id: String::from(id),
            destinations,
            t,
        }
    }
}

type Modules = Vec<ModuleType>;

fn parse(input: impl Iterator<Item = String>) -> Modules {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut last_reserved_id = 2;
    let builder = input.map(ModuleBuilder::from).collect::<Vec<_>>();
    let mut modules: Modules = vec![ModuleType::Empty; builder.len() + 10];

    map.insert(String::from("broadcaster"), 1);
    map.insert(String::from("nx"), 2);

    for b in builder {
        let id = *map.entry(b.id).or_insert_with(|| {
            last_reserved_id += 1;
            last_reserved_id
        });

        let destinations: Vec<usize> = b
            .destinations
            .iter()
            .map(|destination| {
                *map.entry(destination.clone()).or_insert_with(|| {
                    last_reserved_id += 1;
                    last_reserved_id
                })
            })
            .collect::<Vec<_>>();
        modules[id] = match b.t {
            ModuleTypeBuilder::Broadcast => Broadcast { id, destinations }.into(),
            ModuleTypeBuilder::Conjuction => Conjuction {
                id,
                inputs: HashMap::new(),
                destinations,
            }
            .into(),
            ModuleTypeBuilder::FlipFlop => FlipFlop {
                id,
                state: SignalType::Low,
                destinations,
            }
            .into(),
        };
    }

    let destinations: Vec<(usize, Vec<usize>)> = modules
        .iter()
        .map(|m| (m.id(), m.destinations().to_vec()))
        .collect();

    destinations.iter().for_each(|(source, d)| {
        d.iter().for_each(|id| {
            if let ModuleType::Conjuction(ref mut module) = modules[*id] {
                module.add_input(*source);
            }
        });
    });

    modules
}

fn one(modules: &mut Modules) -> usize {
    let mut fifo = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        fifo.push_back(Signal {
            origin: 0,
            destination: 1,
            value: SignalType::Low,
        });

        while let Some(signal) = fifo.pop_front() {
            match signal.value {
                SignalType::Low => low_count += 1,
                SignalType::High => high_count += 1,
            }
            fifo.extend(modules[signal.destination].receive(&signal));
        }
    }
    low_count * high_count
}

fn two(modules: &mut Modules) -> usize {
    let mut fifo: Vec<Signal> = Vec::with_capacity(100000);
    for i in 0..10_000_000 {
        // let now = std::time::Instant::now();
        fifo.clear();
        fifo.push(Signal {
            origin: 0,
            destination: 1,
            value: SignalType::Low,
        });
        let mut n = 0;

        while n < fifo.len() {
            let signal = &fifo[n];
            if signal.value == SignalType::Low && signal.destination == 2 {
                return i;
            }
            fifo.extend(modules[signal.destination].receive(signal));
            n += 1;
        }
        // println!("{}: {:?}", i, now.elapsed());
    }

    0
}

pub fn run(runner: &Runner) {
    let result = match runner.part {
        crate::Part::One => one(&mut parse(get_non_empty_lines(&runner.path))),
        crate::Part::Two => two(&mut parse(get_non_empty_lines(&runner.path))),
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;

    const INPUT_2: &str = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;
    #[test]
    fn test_one1() {
        let mut modules = parse(INPUT_1.trim().lines().map(String::from));
        assert_eq!(one(&mut modules), 32000000);
    }

    #[test]
    fn test_one2() {
        let mut modules = parse(INPUT_2.trim().lines().map(String::from));
        assert_eq!(one(&mut modules), 11687500);
    }
}
