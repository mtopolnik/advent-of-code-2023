use std::{cmp::min, fs::read_to_string};

fn main() {
    let input: Vec<Vec<Vec<u8>>> = read_to_string("input/day13.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.lines().map(|str| str.as_bytes().to_vec()).collect())
        .collect();

    let part1: usize = input.iter().map(|image| find_reflection(image, 0)).sum();
    println!("Part 1: {part1}"); // 37975
    let part2: usize = input.iter().map(|image| find_reflection(image, 1)).sum();
    println!("Part 2: {part2}"); // 32497
}

fn find_reflection(image: &[Vec<u8>], required_smudge_count: usize) -> usize {
    let row_len = image[0].len();
    'outer: for i in 1..image.len() {
        let mut smudge_count = 0;
        for j in 0..min(i, image.len() - i) {
            let upper_row = &image[i - j - 1];
            let lower_row = &image[i + j];
            for k in 0..row_len {
                if upper_row[k] != lower_row[k] {
                    if smudge_count == required_smudge_count {
                        continue 'outer;
                    }
                    smudge_count += 1;
                }
            }
        }
        if smudge_count == required_smudge_count {
            return 100 * i;
        }
    }
    'outer2: for i in 1..row_len {
        let mut smudge_count = 0;
        for j in 0..min(i, row_len - i) {
            for k in 0..image.len() {
                let row = &image[k];
                if row[i + j] != row[i - j - 1] {
                    if smudge_count == required_smudge_count {
                        continue 'outer2;
                    }
                    smudge_count += 1;
                }
            }
        }
        if smudge_count == required_smudge_count {
            return i;
        }
    }
    panic!("No reflection!");
}
