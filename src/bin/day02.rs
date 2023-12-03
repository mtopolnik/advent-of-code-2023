use std::{cmp::max, fs::read_to_string};

use regex::Regex;

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day02.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("part1: {}", part1(&input_lines));
    println!("part2: {}", part2(&input_lines));
}

fn part1(input_lines: &[String]) -> usize {
    let game_re = Regex::new(r"Game (\d+?): ").unwrap();
    let move_re = Regex::new(r"(\d+?) (red|green|blue)").unwrap();

    let mut sum = 0;
    for line in input_lines {
        let header_match = game_re.captures(&line).unwrap();
        let game_id: usize = str::parse(&header_match[1]).unwrap();
        let mut is_possible = true;
        for color_count_str in line[header_match[0].len()..].split([',', ';']) {
            let groups = move_re.captures(&color_count_str).unwrap();
            let count: usize = str::parse(&groups[1]).unwrap();
            let color = &groups[2];
            let limit = match color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => panic!("{color}"),
            };
            if count > limit {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            sum += game_id;
        }
    }
    sum
}

fn part2(input_lines: &[String]) -> usize {
    let move_re = Regex::new(r"(\d+?) (red|green|blue)").unwrap();

    let mut sum = 0;
    for line in input_lines {
        let header_stripped = line.split(": ").last().unwrap();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for color_count_str in header_stripped.split([',', ';']) {
            let groups = move_re.captures(&color_count_str).unwrap();
            let count: usize = str::parse(&groups[1]).unwrap();
            let color = &groups[2];
            match color {
                "red" => max_red = max(max_red, count),
                "green" => max_green = max(max_green, count),
                "blue" => max_blue = max(max_blue, count),
                _ => panic!("{color}"),
            };
        }
        sum += max_red * max_green * max_blue;
    }
    sum
}
