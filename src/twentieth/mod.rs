use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use crate::{utils::get_non_empty_lines, Runner};

#[derive(Debug, Clone, Eq, PartialEq)]
enum SignalType {
    High,
    Low,
}

struct Signal {
    origin: Arc<str>,
    destination: Arc<str>,
    value: SignalType,
}

trait Module {
    fn id(&self) -> Arc<str>;
    fn receive(&mut self, signal: Signal) -> Vec<Signal>;
    fn destinations(&self) -> &[Arc<str>];
}

#[derive(Debug)]
struct Broadcast {
    id: Arc<str>,
    destinations: Vec<Arc<str>>,
}

impl Module for Broadcast {
    fn id(&self) -> Arc<str> {
        self.id.clone()
    }
    fn receive(&mut self, signal: Signal) -> Vec<Signal> {
        self.destinations
            .iter()
            .map(|destination| Signal {
                origin: signal.destination.clone(),
                destination: destination.clone(),
                value: signal.value.clone(),
            })
            .collect()
    }
    fn destinations(&self) -> &[Arc<str>] {
        &self.destinations
    }
}

#[derive(Debug)]
struct Conjuction {
    id: Arc<str>,
    inputs: HashMap<Arc<str>, SignalType>,
    destinations: Vec<Arc<str>>,
}

impl Module for Conjuction {
    fn id(&self) -> Arc<str> {
        self.id.clone()
    }
    fn receive(&mut self, signal: Signal) -> Vec<Signal> {
        self.inputs.insert(signal.origin, signal.value);
        let signal_type = if self.inputs.values().all(|value| value == &SignalType::High) {
            SignalType::Low
        } else {
            SignalType::High
        };

        self.destinations
            .iter()
            .map(|destination| Signal {
                origin: self.id(),
                destination: destination.clone(),
                value: signal_type.clone(),
            })
            .collect()
    }
    fn destinations(&self) -> &[Arc<str>] {
        &self.destinations
    }
}

impl Conjuction {
    fn add_input(&mut self, input: Arc<str>) {
        self.inputs.insert(input, SignalType::Low);
    }
}

#[derive(Debug)]
struct FlipFlop {
    id: Arc<str>,
    state: SignalType,
    destinations: Vec<Arc<str>>,
}

impl Module for FlipFlop {
    fn id(&self) -> Arc<str> {
        self.id.clone()
    }
    fn receive(&mut self, signal: Signal) -> Vec<Signal> {
        if signal.value == SignalType::Low {
            self.state = match self.state {
                SignalType::High => SignalType::Low,
                SignalType::Low => SignalType::High,
            };
            return self
                .destinations
                .iter()
                .map(|destination| Signal {
                    origin: signal.destination.clone(),
                    destination: destination.clone(),
                    value: self.state.clone(),
                })
                .collect();
        }
        vec![]
    }
    fn destinations(&self) -> &[Arc<str>] {
        &self.destinations
    }
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
    Broadcast(Broadcast),
}

impl Module for ModuleType {
    fn id(&self) -> Arc<str> {
        match self {
            ModuleType::FlipFlop(module) => module.id(),
            ModuleType::Conjuction(module) => module.id(),
            ModuleType::Broadcast(module) => module.id(),
        }
    }
    fn receive(&mut self, signal: Signal) -> Vec<Signal> {
        match self {
            ModuleType::FlipFlop(module) => module.receive(signal),
            ModuleType::Conjuction(module) => module.receive(signal),
            ModuleType::Broadcast(module) => module.receive(signal),
        }
    }
    fn destinations(&self) -> &[Arc<str>] {
        match self {
            ModuleType::FlipFlop(module) => module.destinations(),
            ModuleType::Conjuction(module) => module.destinations(),
            ModuleType::Broadcast(module) => module.destinations(),
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

impl From<String> for ModuleType {
    fn from(value: String) -> Self {
        let (id, destinations) = value
            .split_once(" -> ")
            .expect("should have an id and destionation");
        let destinations = destinations.split(", ").map(Arc::from).collect();
        if id == "broadcaster" {
            Broadcast {
                id: Arc::from(id),
                destinations,
            }
            .into()
        } else if let Some(id) = id.strip_prefix('%') {
            FlipFlop {
                id: Arc::from(id),
                state: SignalType::Low,
                destinations,
            }
            .into()
        } else if let Some(id) = id.strip_prefix('&') {
            Conjuction {
                id: Arc::from(id),
                inputs: HashMap::new(),
                destinations,
            }
            .into()
        } else {
            panic!("unknown module type")
        }
    }
}

type Modules = HashMap<Arc<str>, ModuleType>;

fn parse(input: impl Iterator<Item = String>) -> Modules {
    let mut modules = HashMap::new();
    for line in input {
        let module: ModuleType = line.into();
        for destination in module.destinations() {
            modules.entry(destination.clone()).and_modify(|m| {
                if let ModuleType::Conjuction(module) = m {
                    module.add_input(module.id());
                }
            });
        }
        modules.insert(module.id(), module);
    }

    let destinations: Vec<(Arc<str>, Vec<Arc<str>>)> = modules
        .values()
        .map(|m| (m.id(), m.destinations().to_vec()))
        .collect();

    destinations.iter().for_each(|(source, d)| {
        d.iter().for_each(|id| {
            modules.entry(id.clone()).and_modify(|m| {
                if let ModuleType::Conjuction(module) = m {
                    module.add_input(source.clone());
                }
            });
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
            origin: Arc::from("button"),
            destination: Arc::from("broadcaster"),
            value: SignalType::Low,
        });

        while let Some(signal) = fifo.pop_front() {
            match signal.value {
                SignalType::Low => low_count += 1,
                SignalType::High => high_count += 1,
            }
            if let Some(module) = modules.get_mut(&signal.destination) {
                fifo.extend(module.receive(signal));
            }
        }
    }
    low_count * high_count
}

pub fn run(runner: &Runner) {
    let result = match runner.part {
        crate::Part::One => one(&mut parse(get_non_empty_lines(&runner.path))),
        crate::Part::Two => todo!(),
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
