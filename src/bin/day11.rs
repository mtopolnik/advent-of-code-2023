use std::{fs::read_to_string, ops::Range};

#[derive(Copy, Clone)]
enum Weight {
    Normal,
    Expanded,
}
use Weight::*;

fn main() {
    let universe: Vec<Vec<u8>> = read_to_string("input/day11.txt")
        .unwrap()
        .lines()
        .map(|s| s.bytes().collect())
        .collect();

    let row_weights: Vec<Weight> = universe
        .iter()
        .map(|row| {
            if row.iter().all(|&b| b == b'.') {
                Expanded
            } else {
                Normal
            }
        })
        .collect();

    let col_weights: Vec<Weight> = (0..universe[0].len())
        .into_iter()
        .map(|i| {
            if universe.iter().all(|row| row[i] == b'.') {
                Expanded
            } else {
                Normal
            }
        })
        .collect();

    let galaxy_locations = universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, b)| match *b == b'#' {
                    true => Some((x, y)),
                    _ => None,
                })
        })
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        distance_sum(2, &galaxy_locations, &row_weights, &col_weights)
    );
    println!(
        "Part 2: {}",
        distance_sum(1_000_000, &galaxy_locations, &row_weights, &col_weights)
    );
}

fn distance_sum(
    expansion_factor: usize,
    galaxy_locations: &[(usize, usize)],
    row_weights: &[Weight],
    col_weights: &[Weight],
) -> usize {
    galaxy_locations
        .iter()
        .enumerate()
        .flat_map(|(i, &(x1, y1))| {
            galaxy_locations.iter().skip(i + 1).map(move |&(x2, y2)| {
                distance_1d(&row_weights, expansion_factor, y1, y2)
                    + distance_1d(&col_weights, expansion_factor, x1, x2)
            })
        })
        .sum()
}

fn distance_1d(
    data: &[Weight],
    expansion_factor: usize,
    start: usize,
    destination: usize,
) -> usize {
    data[to_range(start, destination)]
        .iter()
        .map(|w| match w {
            Normal => 1,
            Expanded => expansion_factor,
        })
        .sum()
}

fn to_range(n1: usize, n2: usize) -> Range<usize> {
    if n1 <= n2 {
        n1..n2
    } else {
        n2..n1
    }
}
