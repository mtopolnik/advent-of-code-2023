use std::fs::read_to_string;

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day09.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let (part1, part2) = input_lines
        .into_iter()
        .map(parse_vec)
        .map(diff_seqs_first_last)
        .map(|dsfl| (future(&dsfl), past(&dsfl)))
        .reduce(|(future1, past1), (future2, past2)| (future1 + future2, past1 + past2))
        .unwrap();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn parse_vec(line: String) -> Vec<i64> {
    line.split(' ')
        .map(|num_str| str::parse(num_str).unwrap())
        .collect()
}

fn future(diff_seqs_first_last: &Vec<(i64, i64)>) -> i64 {
    diff_seqs_first_last.into_iter().map(|(_, last)| last).sum()
}

fn past(diff_seqs_first_last: &Vec<(i64, i64)>) -> i64 {
    diff_seqs_first_last
        .into_iter()
        .rev()
        .fold(0, |acc, (first, _)| first - acc)
}

fn diff_seqs_first_last(mut seq: Vec<i64>) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    while seq.iter().any(|n| *n != 0) {
        let diff_seq = diff_seq(&seq);
        result.push((*seq.first().unwrap(), *seq.last().unwrap()));
        seq = diff_seq;
    }
    result
}

fn diff_seq(seq: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    if seq.is_empty() {
        return result;
    }
    let mut prev = seq[0];
    for num in seq.iter().skip(1) {
        result.push(num - prev);
        prev = *num;
    }
    result
}
