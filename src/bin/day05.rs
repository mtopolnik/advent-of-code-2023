use std::fs::read_to_string;
use std::sync::OnceLock;

use regex::Regex;

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day05.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let seeds_part1: Vec<i64> = num_re()
        .find_iter(input_lines[0].trim())
        .map(|m| str::parse::<i64>(&input_lines[0][m.start()..m.end()]).unwrap())
        .collect();

    let seeds_part2_iter = (0..seeds_part1.len() / 2).flat_map(|i| {
        let base = seeds_part1[2 * i];
        let len = seeds_part1[2 * i + 1];
        (base..base + len).into_iter()
    });

    let mappings = load_mappings(input_lines);

    println!(
        "{}",
        find_nearest_location(seeds_part1.iter().map(|x| *x), &mappings)
    );
    println!("{}", find_nearest_location(seeds_part2_iter, &mappings));
}

fn num_re() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\d+").unwrap())
}

fn load_mappings(input_lines: Vec<String>) -> Vec<Mapping> {
    let mut mappings = Vec::<Mapping>::new();
    let mut curr_mapping = Mapping::default();
    let mut i = 1;
    loop {
        if (i >= input_lines.len()) || input_lines[i].trim().is_empty() {
            i += 2;
            if !curr_mapping.ranges.is_empty() {
                mappings.push(curr_mapping);
                curr_mapping = Mapping::default();
            }
        }
        if i >= input_lines.len() {
            break;
        }

        let mut num_strs = parse_vec(input_lines[i].trim()).into_iter();
        curr_mapping.ranges.push(RangeMap {
            dest: num_strs.next().unwrap(),
            src: num_strs.next().unwrap(),
            len: num_strs.next().unwrap(),
        });
        i += 1;
    }
    mappings
}

fn parse_vec(line: &str) -> Vec<i64> {
    num_re()
        .find_iter(line)
        .map(|m| str::parse(&line[m.start()..m.end()]).unwrap())
        .collect()
}

fn find_nearest_location(seeds: impl Iterator<Item = i64>, mappings: &[Mapping]) -> i64 {
    seeds
        .map(|seed_id| map_seed_to_location(seed_id, &mappings))
        .min()
        .unwrap()
}

fn map_seed_to_location(seed_id: i64, mappings: &[Mapping]) -> i64 {
    let mut id = seed_id;
    for mapping in mappings {
        id = mapping.map_src_to_dest(id);
    }
    id
}

#[derive(Default)]
struct Mapping {
    pub ranges: Vec<RangeMap>,
}

#[derive(Default)]
struct RangeMap {
    dest: i64,
    src: i64,
    len: i64,
}

impl Mapping {
    pub fn map_src_to_dest(&self, src: i64) -> i64 {
        for range_map in &self.ranges {
            let offset = src - range_map.src;
            if offset >= 0 && offset < range_map.len {
                return range_map.dest + offset;
            }
        }
        return src;
    }
}
