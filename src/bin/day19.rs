use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

use regex::Regex;

type Part = [u16; 4];
type PartChoice = [(u16, u16); 4];

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

    fn negate(&self) -> Rule {
        if self.ord == Ordering::Less {
            Rule {
                ord: Ordering::Greater,
                limit: self.limit - 1,
                category: self.category,
                on_match: self.on_match.clone(),
            }
        } else {
            Rule {
                ord: Ordering::Less,
                limit: self.limit + 1,
                category: self.category,
                on_match: self.on_match.clone(),
            }
        }
    }
}

#[derive(Clone, Copy)]
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

    println!("Part 1: {}", part1(&parts, &workflows)); // 346230
    println!("Part 2: {}", part2(workflows)); // 124693661917133
}

fn part1(parts: &[[u16; 4]], workflows: &HashMap<String, Workflow>) -> usize {
    let mut accepted: Vec<Part> = Vec::new();
    'part_loop: for part in parts {
        let mut workflow = &workflows["in"];
        'workflow_loop: loop {
            let outcome = workflow
                .rules
                .iter()
                .filter_map(|rule| rule.apply(*part))
                .next()
                .unwrap_or(&workflow.fallback);
            match outcome {
                "R" => continue 'part_loop,
                "A" => {
                    accepted.push(*part);
                    continue 'part_loop;
                }
                next_workflow => {
                    workflow = &workflows[next_workflow];
                    continue 'workflow_loop;
                }
            }
        }
    }
    accepted
        .into_iter()
        .map(|part| part.into_iter().map(|n| n as usize).sum::<usize>())
        .sum::<usize>()
}

fn part2(workflows: HashMap<String, Workflow>) -> usize {
    let start_wf = workflows.get("in").unwrap();
    let mut todo_list: Vec<(&Workflow, PartChoice)> = Vec::new();
    todo_list.push((start_wf, [(1, 4000), (1, 4000), (1, 4000), (1, 4000)]));
    let mut combination_count = 0_usize;
    loop {
        let Some((wf, mut part_choice)) = todo_list.pop() else {
            break;
        };
        for rule in wf.rules.iter() {
            let restricted_on_match = restrict_choice(part_choice, &rule);
            match &*rule.on_match {
                "R" => {}
                "A" => combination_count += count_combinations(restricted_on_match),
                other => {
                    todo_list.push((workflows.get(other).unwrap(), restricted_on_match));
                }
            };
            part_choice = restrict_choice(part_choice, &rule.negate());
        }
        match &*wf.fallback {
            "R" => {}
            "A" => combination_count += count_combinations(part_choice),
            other => todo_list.push((workflows.get(other).unwrap(), part_choice)),
        };
    }
    combination_count
}

fn count_combinations(part_choice: PartChoice) -> usize {
    part_choice
        .into_iter()
        .map(|(low, high)| (high - low + 1) as usize)
        .reduce(|acc, count| acc * count)
        .unwrap()
}

fn restrict_choice(mut part_choice: PartChoice, rule: &Rule) -> PartChoice {
    let Rule { category, ord, limit, .. } = rule;
    let (low, high) = &mut part_choice[usize::from(category)];
    match ord {
        Ordering::Less => *high = limit - 1,
        Ordering::Greater => *low = limit + 1,
        Ordering::Equal => panic!(),
    };
    part_choice
}
