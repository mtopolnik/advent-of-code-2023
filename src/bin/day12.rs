use std::fs::read_to_string;
use std::sync::OnceLock;

use memoize::memoize;

type Count = usize;
type AsciiChar = u8;

fn main() {
    fn unfold_string(input: &str) -> Vec<u8> {
        // input
        format!("{input}?{input}?{input}?{input}?{input}")
            .as_bytes()
            .to_vec()
    }

    fn unfold_num(input: &[Count]) -> Vec<Count> {
        // input.to_vec()
        [input, input, input, input, input].concat().to_vec()
    }

    static INPUT: OnceLock<Vec<(Vec<AsciiChar>, Vec<Count>)>> = OnceLock::new();
    let input = INPUT.get_or_init(|| {
        read_to_string("input/day12.txt")
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
            .collect()
    });
    let result: usize = input
        .iter()
        .map(|(record, group_sizes)| {
            let result = num_group_placements(record, group_sizes);
            println!(
                "{} {group_sizes:?} -> {result}",
                String::from_utf8_lossy(record)
            );
            memoized_flush_num_group_placements();
            result
        })
        .sum();
    println!("Answer: {result}");
    // 7506, 548241300348335
}

#[memoize]
fn num_group_placements(record: &'static [AsciiChar], group_sizes: &'static [Count]) -> usize {
    let group_size = group_sizes[0];
    let mut count = 0;
    let min_leftover_size =
        group_size + group_sizes[1..].iter().map(|size| size + 1).sum::<usize>();
    let mut hashtag_encountered = false;
    'outer: for i in 0..=record.len() - min_leftover_size {
        if hashtag_encountered {
            break;
        }
        hashtag_encountered = record[i] == b'#';
        for j in 0..group_size {
            if record[i + j] == b'.' {
                continue 'outer;
            }
        }
        if i + group_size < record.len() && record[i + group_size] == b'#' {
            continue;
        }
        let tail_groups = &group_sizes[1..];
        let tail_index = i + group_size + 1;
        let tail_record = if tail_index <= record.len() {
            &record[tail_index..]
        } else {
            &record[0..0]
        };
        if tail_groups.is_empty() {
            if !tail_record.contains(&b'#') {
                count += 1;
            }
            continue;
        }
        if !tail_record.is_empty() {
            count += num_group_placements(tail_record, &tail_groups);
        }
    }
    count
}
