use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day08.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let instructions: Vec<u8> = input_lines[0].bytes().collect();
    let map: HashMap<String, (String, String)> = input_lines
        .into_iter()
        .skip(2)
        .map(|line| {
            (
                line[0..=2].to_owned(),
                (line[7..=9].to_owned(), line[12..=14].to_owned()),
            )
        })
        .collect();

    part1(&instructions, &map);
    part2(&instructions, &map);
}

fn part1(instructions: &[u8], map: &HashMap<String, (String, String)>) {
    println!(
        "Part 1: {}",
        path_length("AAA", instructions, map, |node| node == "ZZZ")
    );
}

fn part2(instructions: &[u8], map: &HashMap<String, (String, String)>) {
    let start_nodes: Vec<&String> = map
        .keys()
        .filter(|&node| node.bytes().nth(2).unwrap() == b'A')
        .collect();
    let instruction_cycle_counts: Vec<usize> = start_nodes
        .into_iter()
        .map(|start| {
            path_length(start, instructions, map, |node| {
                node.bytes().nth(2).unwrap() == b'Z'
            }) / instructions.len()
        })
        .collect();
    let result = instruction_cycle_counts
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap()
        * instructions.len();
    println!("part 2 result: {result}");
}

fn path_length(
    start: &str,
    instructions: &[u8],
    map: &HashMap<String, (String, String)>,
    stop_fn: impl Fn(&str) -> bool,
) -> usize {
    let mut curr = start;
    let mut forever_instructions = instructions.iter().cycle();
    let mut move_count = 0;
    while !stop_fn(curr) {
        let instruction = *forever_instructions.next().unwrap();
        let directions = &map[curr];
        curr = if instruction == b'L' {
            &directions.0
        } else {
            &directions.1
        };
        move_count += 1;
    }
    move_count
}
