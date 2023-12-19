use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

use regex::Regex;

type Part = [u16; 4];

struct Workflow {
    rules: Vec<Rule>,
    fallback: String,
}

struct Rule {
    category: Category,
    ord: Ordering,
    limit: u16,
    on_match: String,
}

impl Rule {
    fn apply(&self, part: Part) -> Option<&str> {
        let rating: u16 = part[usize::from(&self.category)];
        (rating.cmp(&self.limit) == self.ord).then_some(&self.on_match)
    }
}

enum Category {
    X,
    M,
    A,
    S,
}
use Category::*;

impl From<&Category> for usize {
    fn from(value: &Category) -> Self {
        match value {
            X => 0,
            M => 1,
            A => 2,
            S => 3,
        }
    }
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!("Not a category: {value}"),
        }
    }
}

fn ordering_from(symbol: &str) -> Ordering {
    match symbol {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => panic!("Not an ordering: {symbol}"),
    }
}

fn main() {
    let input = &read_to_string("input/day19.txt").unwrap();
    let (workflow_str, parts_str) = input.split_once("\n\n").unwrap();
    let rule_re = Regex::new(r"([a-z]+)([<>])(\d+):([a-zAR]+)").unwrap();
    let fallback_re = Regex::new(r",([a-zAR]+)\}$").unwrap();
    let limit_re = Regex::new(r"\d+").unwrap();

    let workflows: HashMap<String, Workflow> = workflow_str
        .lines()
        .map(|line| {
            let open_brace_pos = line.find("{").unwrap();
            let workflow_name = line[0..open_brace_pos].to_string();
            let rules_str = &line[open_brace_pos + 1..];
            let rules: Vec<Rule> = rule_re
                .captures_iter(rules_str)
                .map(|groups| Rule {
                    category: groups[1].into(),
                    ord: ordering_from(&groups[2]),
                    limit: groups[3].parse().unwrap(),
                    on_match: groups[4].to_string(),
                })
                .collect();
            let fallback = fallback_re.captures(rules_str).unwrap()[1].to_string();
            (workflow_name, Workflow { rules, fallback })
        })
        .collect();
    let parts: Vec<Part> = parts_str
        .lines()
        .map(|line| {
            let matches: Vec<u16> = limit_re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            [matches[0], matches[1], matches[2], matches[3]]
        })
        .collect();

    let mut accepted: Vec<Part> = Vec::new();
    'part_loop: for part in parts {
        let mut workflow = &workflows["in"];
        'workflow_loop: loop {
            let outcome = workflow
                .rules
                .iter()
                .filter_map(|rule| rule.apply(part))
                .next()
                .unwrap_or(&workflow.fallback);
            match outcome {
                "R" => continue 'part_loop,
                "A" => {
                    accepted.push(part);
                    continue 'part_loop;
                }
                next_workflow => {
                    workflow = &workflows[next_workflow];
                    continue 'workflow_loop;
                }
            }
        }
    }
    let result = accepted
        .into_iter()
        .map(|part| part.into_iter().map(|n| n as usize).sum::<usize>())
        .sum::<usize>();
    println!("Part 1: {result}");
}
