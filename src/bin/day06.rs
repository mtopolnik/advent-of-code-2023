use std::fs::read_to_string;
use std::sync::OnceLock;

use regex::Regex;

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day06.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let times = parse_vec(&input_lines[0]);
    let distances = parse_vec(&input_lines[1]);
    let mut result = 1;
    for i in 0..times.len() {
        let race_time = times[i] as f64;
        let record_distance = distances[i] as f64;
        result *= num_choices(race_time, record_distance);
    }
    println!("part 1: {result}");

    let race_time = concat_parse_num(&input_lines[0]);
    let record_distance = concat_parse_num(&input_lines[1]);
    let num_choices = num_choices(race_time, record_distance);
    println!("part 2: {num_choices}");
}

fn num_choices(race_time: f64, record_distance: f64) -> u64 {
    let discriminant = (race_time * race_time - 4.0 * record_distance).sqrt();

    let match_record_time_low = (race_time - discriminant) / 2.0;
    let charge_time_low = match_record_time_low.floor() as u64 + 1;

    let match_record_time_high = (race_time + discriminant) / 2.0;
    let charge_time_high = match_record_time_high.ceil() as u64 - 1;

    charge_time_high - charge_time_low + 1
}

fn parse_vec(line: &str) -> Vec<u64> {
    num_re()
        .find_iter(line)
        .map(|m| str::parse(&line[m.start()..m.end()]).unwrap())
        .collect()
}

fn concat_parse_num(line: &str) -> f64 {
    str::parse(
        &num_re()
            .find_iter(line)
            .map(|m| m.as_str())
            .collect::<String>(),
    )
    .unwrap()
}

fn num_re() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\d+").unwrap())
}
