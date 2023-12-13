use std::{cmp::min, fs::read_to_string, io::Write, time::Instant};

type Count = usize;
type AsciiChar = u8;

fn main() {
    fn unfold_string(input: &str) -> Vec<u8> {
        input
            // format!("{input}?{input}?{input}?{input}?{input}")
            .as_bytes()
            .to_vec()
    }

    fn unfold_num(input: &[Count]) -> Vec<Count> {
        input.to_vec()
        // [input, input, input, input, input].concat().to_vec()
    }

    let input: Vec<(Vec<AsciiChar>, Vec<Count>)> = read_to_string("input/day12.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let mut iter = s.split(" ");
            (
                unfold_string(iter.next().unwrap()),
                unfold_num(
                    &iter
                        .next()
                        .unwrap()
                        .split(",")
                        .map(|s| str::parse::<Count>(s).unwrap())
                        .collect::<Vec<Count>>(),
                ),
            )
        })
        .collect();
    let result: usize = input
        .iter()
        .map(|(record, group_sizes)| {
            let x = num_arrangements(record, group_sizes);
            println!("num_arrangements {x}");
            x
        })
        .sum();
    println!("Part 1: {result}");
}

fn num_arrangements(record: &[AsciiChar], group_sizes: &[Count]) -> usize {
    let start = Instant::now();
    let groups_plus_gaps = group_sizes
        .iter()
        .map(|size| *size as Count + 1)
        .sum::<Count>();
    let extra_gap_count = record.len() as Count - (groups_plus_gaps - 1);
    let placement_count = group_sizes.len() as Count + 1;
    let arrangement_count =
        num_integer::binomial(extra_gap_count + placement_count - 1, placement_count - 1);
    println!(
        "record {}, group_sizes {group_sizes:?}, extra_gap_count {extra_gap_count}, \
        placement_count {placement_count}, total_num_arrangements {arrangement_count}",
        String::from_utf8_lossy(record)
    );
    let mut num_valid_arrangements = 0;
    for gap_arrangement in gap_arrangements(extra_gap_count, placement_count, &group_sizes, &record)
    {
        if is_valid_arrangement(&group_sizes, &gap_arrangement, &record) {
            num_valid_arrangements += 1;
        }
    }
    println!(
        "valid arrangements: {num_valid_arrangements}, elapsed: {:?}",
        start.elapsed()
    );
    num_valid_arrangements
}

fn is_valid_arrangement(
    group_sizes: &[Count],
    gap_arrangement: &[Count],
    record: &[AsciiChar],
) -> bool {
    let mut rec_index = 0;
    for i in 0..group_sizes.len() {
        for _ in 0..gap_arrangement[i] {
            if record[rec_index] == b'#' {
                return false;
            }
            rec_index += 1;
        }
        for _ in 0..group_sizes[i] {
            if record[rec_index] == b'.' {
                return false;
            }
            rec_index += 1;
        }
        if i < group_sizes.len() - 1 {
            // Check for the mandatory gap between groups, not accounted for in gap_arrangement
            if record[rec_index] == b'#' {
                return false;
            }
            rec_index += 1;
        }
    }
    if gap_arrangement.len() > group_sizes.len() {
        for _ in 0..*gap_arrangement.last().unwrap() {
            if record[rec_index] == b'#' {
                return false;
            }
            rec_index += 1;
        }
    }
    true
}

fn gap_arrangements(
    num_items: Count,
    num_buckets: Count,
    group_sizes: &[Count],
    record: &[AsciiChar],
) -> Vec<Vec<Count>> {
    if num_buckets == 1 {
        let gap_arrangement = vec![num_items as Count];
        if is_valid_arrangement(&group_sizes[0..1], &gap_arrangement, &record) {
            return vec![gap_arrangement];
        } else {
            return Vec::new();
        }
    }
    let mut result = Vec::new();
    for items_in_last_bucket in 0..=num_items {
        let arrangements_for_front = gap_arrangements(
            num_items - items_in_last_bucket,
            num_buckets - 1,
            group_sizes,
            record,
        );
        for mut gap_arrangement in arrangements_for_front {
            gap_arrangement.push(items_in_last_bucket as Count);
            if is_valid_arrangement(
                &group_sizes[0..min(gap_arrangement.len(), group_sizes.len())],
                &gap_arrangement,
                &record,
            ) {
                result.push(gap_arrangement);
            }
        }
    }
    return result;
}
