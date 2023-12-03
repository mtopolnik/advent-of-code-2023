use std::fs::read_to_string;
use std::{collections::HashMap, iter::repeat, sync::OnceLock};

use regex::Regex;

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day03.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let empty_line = repeat(".")
        .take(input_lines[0].len() + 2)
        .collect::<String>();
    let mut lines = vec![empty_line.clone()];
    for in_line in input_lines {
        lines.push(format!(".{}.", in_line));
    }
    lines.push(empty_line);

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[String]) -> usize {
    let num_re = num_re();

    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        for m in num_re.find_iter(&line) {
            for (x, y) in neighbor_places(m.start(), m.end(), y) {
                let neighbor = lines[y].as_bytes()[x];
                if is_symbol(neighbor) {
                    sum += str::parse::<usize>(&line[m.start()..m.end()]).unwrap();
                    break;
                }
            }
        }
    }
    sum
}

fn part2(lines: &[String]) -> usize {
    let num_re = num_re();

    let mut gear_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for m in num_re.find_iter(&line) {
            for (x, y) in neighbor_places(m.start(), m.end(), y) {
                let line_bytes = lines[y].as_bytes();
                let neighbor = line_bytes[x];
                if neighbor == b'*' {
                    let part_number = str::parse::<usize>(&line[m.start()..m.end()]).unwrap();
                    gear_map.entry((x, y)).or_default().push(part_number);
                }
            }
        }
    }
    let mut sum = 0;
    for geared_parts in gear_map.values() {
        if geared_parts.len() == 2 {
            sum += geared_parts[0] * geared_parts[1];
        }
    }
    sum
}

fn num_re() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\d+").unwrap())
}

fn neighbor_places(start_x: usize, end_x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbor_places = (start_x - 1..end_x + 1)
        .flat_map(|x| vec![(x, y - 1), (x, y + 1)])
        .collect::<Vec<_>>();
    neighbor_places.push((start_x - 1, y));
    neighbor_places.push((end_x, y));
    neighbor_places
}

fn is_symbol(ch: u8) -> bool {
    (ch < b'0' || ch > b'9') && ch != b'.'
}
