use std::{collections::HashMap, fs::read_to_string, sync::Arc, usize};

use crate::Runner;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    GreaterThan,
    LessThan,
}

impl From<char> for Op {
    fn from(val: char) -> Self {
        match val {
            '>' => Op::GreaterThan,
            '<' => Op::LessThan,
            _ => panic!("Invalid op"),
        }
    }
}

#[derive(Debug)]
enum WorkflowResult {
    Accept,
    Reject,
}

impl From<char> for WorkflowResult {
    fn from(val: char) -> Self {
        match val {
            'A' => WorkflowResult::Accept,
            'R' => WorkflowResult::Reject,
            _ => panic!("Invalid result"),
        }
    }
}
#[derive(Debug)]
enum Target {
    Rule(Arc<str>),
    Result(WorkflowResult),
}

impl From<&str> for Target {
    fn from(val: &str) -> Self {
        match val {
            "A" => Target::Result(WorkflowResult::Accept),
            "R" => Target::Result(WorkflowResult::Reject),
            _ => Target::Rule(val.into()),
        }
    }
}

#[derive(Debug)]
enum RuleType {
    Rule(Rule),
    Fallback(Target),
}

impl From<&str> for RuleType {
    fn from(val: &str) -> Self {
        if let Some((rule, target)) = val.split_once(':') {
            let mut c = rule.chars();
            RuleType::Rule(Rule {
                target: target.into(),
                field: c.next().expect("should have a field"),
                op: c.next().expect("should have an op").into(),
                value: rule[2..].parse().expect("should be a number"),
            })
        } else {
            RuleType::Fallback(val.into())
        }
    }
}

#[derive(Debug)]
struct Rule {
    target: Target,
    field: char,
    op: Op,
    value: usize,
}

type Rules = HashMap<Arc<str>, Vec<RuleType>>;

fn parse_rules(rules: &str) -> Rules {
    rules
        .lines()
        .map(|line| {
            let (name, rules) = line.split_once('{').expect("should have a name");
            (
                Arc::from(name),
                rules[..rules.len() - 1]
                    .split(',')
                    .map(RuleType::from)
                    .collect(),
            )
        })
        .fold(HashMap::new(), |mut acc, (name, rules)| {
            acc.insert(name, rules);
            acc
        })
}

fn parse_parts(parts: &str) -> Vec<Vec<(char, usize)>> {
    parts
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(',')
                .map(|part| {
                    let (field, value) = part.split_once('=').expect("should have a name");
                    (
                        field.chars().next().expect("should have a field"),
                        value.parse().expect("should be a number"),
                    )
                })
                .collect()
        })
        .collect()
}

fn parse(str: &str) -> (Rules, Vec<Vec<(char, usize)>>) {
    let (rules, parts) = str
        .trim()
        .split_once("\n\n")
        .expect("input to have two parts");

    (parse_rules(rules), parse_parts(parts))
}

fn acceptnce(part: &[(char, usize)], rules: &Rules) -> bool {
    let mut key = Arc::from("in");
    while let Some(rule) = rules.get(&key) {
        let target = rule
            .iter()
            .find_map(|rule| match rule {
                RuleType::Rule(rule) => {
                    if part.iter().any(|(field, value)| {
                        rule.field == *field
                            && match rule.op {
                                Op::GreaterThan => value > &rule.value,
                                Op::LessThan => value < &rule.value,
                            }
                    }) {
                        Some(&rule.target)
                    } else {
                        None
                    }
                }
                RuleType::Fallback(target) => Some(target),
            })
            .expect("should have a target");
        match target {
            Target::Result(WorkflowResult::Accept) => return true,
            Target::Result(WorkflowResult::Reject) => return false,
            Target::Rule(rule) => key = rule.clone(),
        }
    }
    false
}

fn get_rating(part: &[(char, usize)]) -> usize {
    part.iter().map(|(_, value)| value).sum()
}

fn one(input: &str) -> usize {
    let (rules, parts) = parse(input);
    parts
        .iter()
        .filter(|part| acceptnce(part, &rules))
        .map(|p| get_rating(p))
        .sum()
}

#[derive(Clone, Debug)]
struct Range {
    left: usize,
    right: usize,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            left: 1,
            right: 4000,
        }
    }
}

#[derive(Clone, Debug)]
struct Ranges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl Ranges {
    fn get_filed_mut(&mut self, field: char) -> &mut Range {
        match field {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!("Invalid field"),
        }
    }

    fn split_on(&mut self, rule: &Rule) -> Option<Self> {
        let mut new_range = self.clone();
        let filed = self.get_filed_mut(rule.field);
        if rule.op == Op::GreaterThan && rule.value > filed.left && rule.value <= filed.right {
            filed.right = rule.value;
            new_range.get_filed_mut(rule.field).left = rule.value + 1;
            return Some(new_range);
        } else if rule.op == Op::LessThan && rule.value < filed.right && rule.value >= filed.left {
            filed.left = rule.value;
            new_range.get_filed_mut(rule.field).right = rule.value - 1;
            return Some(new_range);
        }
        None
    }

    fn permutations_count(&self) -> usize {
        (self.x.right - self.x.left + 1)
            * (self.m.right - self.m.left + 1)
            * (self.a.right - self.a.left + 1)
            * (self.s.right - self.s.left + 1)
    }
}

struct QueueItem {
    key: Arc<str>,
    current_ranges: Ranges,
}

enum NextType {
    QueueItem(QueueItem),
    Range(Ranges),
}

impl QueueItem {
    fn next(&self, rules: &Rules) -> Vec<NextType> {
        let mut res = vec![];
        if let Some(rule) = rules.get(&self.key) {
            let mut left = self.current_ranges.clone();
            for r in rule {
                match r {
                    RuleType::Rule(rule) => {
                        if let Some(right) = left.split_on(rule) {
                            match &rule.target {
                                Target::Result(WorkflowResult::Accept) => {
                                    res.push(NextType::Range(right.clone()))
                                }
                                Target::Rule(r) => res.push(NextType::QueueItem(QueueItem {
                                    key: r.clone(),
                                    current_ranges: right.clone(),
                                })),
                                _ => {}
                            }
                        }
                    }
                    RuleType::Fallback(target) => match target {
                        Target::Result(WorkflowResult::Accept) => {
                            res.push(NextType::Range(left.clone()))
                        }
                        Target::Rule(rule) => res.push(NextType::QueueItem(QueueItem {
                            key: rule.clone(),
                            current_ranges: left.clone(),
                        })),
                        _ => {}
                    },
                }
            }
        }
        res
    }
}

fn two(input: &str) -> usize {
    let (rules, _) = parse(input);
    let mut res: Vec<Ranges> = vec![];
    let mut queue: Vec<QueueItem> = vec![QueueItem {
        key: Arc::from("in"),
        current_ranges: Ranges {
            x: Range::default(),
            m: Range::default(),
            a: Range::default(),
            s: Range::default(),
        },
    }];

    while let Some(item) = queue.pop() {
        let next = item.next(&rules);
        for n in next {
            match n {
                NextType::QueueItem(item) => queue.push(item),
                NextType::Range(ranges) => res.push(ranges),
            }
        }
    }

    res.iter().map(|r| r.permutations_count()).sum::<usize>()
}

pub fn run(runner: &Runner) {
    let res = match runner.part {
        crate::Part::One => one(read_to_string(&runner.path)
            .expect("Should have been able to read the file")
            .as_str()),
        crate::Part::Two => two(read_to_string(&runner.path)
            .expect("Should have been able to read the file")
            .as_str()),
    };
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

    #[test]
    fn test_acceptance() {
        let (rules, parts) = parse(TEST_INPUT);
        let accepted = parts
            .iter()
            .filter(|part| acceptnce(part, &rules).clone())
            .collect::<Vec<_>>();
        assert_eq!(accepted.len(), 3);
        assert_eq!(get_rating(&accepted[0]), 7_540);
        assert_eq!(get_rating(&accepted[1]), 4_623);
        assert_eq!(get_rating(&accepted[2]), 6_951);
    }

    #[test]
    fn test_one() {
        assert_eq!(one(TEST_INPUT), 19_114);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(TEST_INPUT), 167409079868000);
    }
}
