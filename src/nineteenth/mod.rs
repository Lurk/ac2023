use std::{collections::HashMap, fs::read_to_string, sync::Arc, usize};

use crate::{utils::get_lines, Runner};

#[derive(Debug)]
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

fn parse_rules(rules: &str) -> HashMap<Arc<str>, Vec<RuleType>> {
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

fn parse(str: &str) -> (HashMap<Arc<str>, Vec<RuleType>>, Vec<Vec<(char, usize)>>) {
    let (rules, parts) = str
        .trim()
        .split_once("\n\n")
        .expect("input to have two parts");

    let rules: HashMap<Arc<str>, Vec<RuleType>> = parse_rules(rules);
    let parts: Vec<Vec<(char, usize)>> = parse_parts(parts);
    (rules, parts)
}

fn acceptnce(part: &[(char, usize)], rules: &HashMap<Arc<str>, Vec<RuleType>>) -> bool {
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

pub fn run(runner: &Runner) {
    let res = match runner.part {
        crate::Part::One => one(read_to_string(&runner.path)
            .expect("Should have been able to read the file")
            .as_str()),
        crate::Part::Two => todo!(),
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
}
