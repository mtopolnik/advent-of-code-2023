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

#[derive(Debug)]
struct SearchState<'a> {
    workflow_name: &'a str,
    part_choice: PartChoice,
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

    part1(&parts, &workflows);

    // workflow_name -> vector of (back_workflow_name, back_workflow, rule_index) sending to it
    let mut back_map: HashMap<&str, Vec<(&str, &Workflow, u8)>> = HashMap::new();
    let mut todo_list: Vec<SearchState> = Vec::new();
    let mut accepted_choices: Vec<PartChoice> = Vec::new();
    for (workflow_name, workflow) in workflows.iter() {
        let mut part_choice: PartChoice = [(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
        for (i, rule) in workflow.rules.iter().enumerate() {
            match &*rule.on_match {
                "A" => {
                    let search_state = SearchState {
                        workflow_name,
                        part_choice: restrict_choice(part_choice, &rule),
                    };
                    todo_list.push(search_state);
                }
                "R" => {}
                other => {
                    back_map
                        .entry(other)
                        .or_default()
                        .push((workflow_name, workflow, i as u8));
                }
            }
            part_choice = restrict_choice(part_choice, &rule.negate());
        }
        match &*workflow.fallback {
            "A" => {
                todo_list.push(SearchState { workflow_name, part_choice });
            }
            "R" => {}
            other => {
                back_map.entry(other).or_default().push((
                    workflow_name,
                    workflow,
                    workflow.rules.len() as u8,
                ));
            }
        }
    }
    loop {
        let Some(SearchState { workflow_name, mut part_choice }) = todo_list.pop() else {
            break;
        };
        if workflow_name == "in" {
            accepted_choices.push(part_choice);
            continue;
        }
        for (back_workflow_name, back_workflow, rule_pos) in back_map.get(&*workflow_name).unwrap()
        {
            for (i, rule) in back_workflow.rules.iter().enumerate() {
                if i as u8 == *rule_pos {
                    let search_state = SearchState {
                        workflow_name: back_workflow_name,
                        part_choice: restrict_choice(part_choice, &rule),
                    };
                    todo_list.push(search_state);
                    break;
                } else {
                    part_choice = restrict_choice(part_choice, &rule.negate());
                }
            }
            if *rule_pos == back_workflow.rules.len() as u8 {
                let search_state = SearchState { workflow_name: back_workflow_name, part_choice };
                todo_list.push(search_state);
            }
        }
    }
    for choice in accepted_choices {
        println!("{choice:?}");
    }
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

fn part1(parts: &[[u16; 4]], workflows: &HashMap<String, Workflow>) {
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
    let result = accepted
        .into_iter()
        .map(|part| part.into_iter().map(|n| n as usize).sum::<usize>())
        .sum::<usize>();
    println!("Part 1: {result}");
}
