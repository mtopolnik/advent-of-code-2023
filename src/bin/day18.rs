use std::{fs::read_to_string, iter::repeat, thread::sleep, time::Duration};

fn main() {
    let input = &read_to_string("input/day18.txt").unwrap();
    let instructions: Vec<(Direction, usize)> = input
        .lines()
        .map(|line| {
            let [dir_str, count_str, ..] = line.split(" ").collect::<Vec<_>>()[..] else {
                panic!("Parse error");
            };
            (Direction::parse(dir_str), str::parse(count_str).unwrap())
        })
        .collect();
    let (ground, start_y, start_x) = build_trench(&instructions);
    let ground = dig_out_interior(ground);
    debug_print(&ground, start_y, start_x);
    let dugout_block_count = ground
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&block| block == b'#')
        .count();
    println!("Part 1: {dugout_block_count}"); // 53844
}

fn build_trench(instructions: &[(Direction, usize)]) -> (Vec<Vec<u8>>, isize, isize) {
    let mut ground = Vec::<Vec<u8>>::new();
    ground.push("...".as_bytes().to_vec());
    ground.push(".#.".as_bytes().to_vec());
    ground.push("...".as_bytes().to_vec());
    let (mut start_y, mut start_x) = (1_isize, 1_isize);
    let (mut y, mut x) = (start_y, start_x);
    for &(dir, count) in instructions {
        let (delta_y, delta_x) = dir.delta();
        for _ in 0..count {
            (y, x) = (y + delta_y, x + delta_x);
            ground[y as usize][x as usize] = b'#';
            if y == 0 {
                ground.insert(0, repeat(b'.').take(ground[0].len()).collect());
                y = 1;
                start_y += 1;
            } else if y == ground.len() as isize - 1 {
                ground.push(repeat(b'.').take(ground[0].len()).collect());
            } else if x == 0 {
                for i in 0..ground.len() {
                    ground[i].insert(0, b'.');
                }
                x = 1;
                start_x += 1;
            } else if x == ground[0].len() as isize - 1 {
                for i in 0..ground.len() {
                    ground[i].push(b'.');
                }
            }
        }
    }
    (ground, start_y, start_x)
}

fn dig_out_interior(mut ground: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let trench_map = ground.clone();
    for y in 1..trench_map.len() - 1 {
        let mut inside = false;
        let mut expected_edge = Some((b'#', b'#', b'#'));
        expected_edge = None;
        for x in 1..trench_map[0].len() - 1 {
            let edge = (trench_map[y - 1][x], trench_map[y][x], trench_map[y + 1][x]);
            match edge {
                (_, b'.', _) => {
                    if inside {
                        ground[y][x] = b'#';
                    }
                }
                (b'#', b'#', b'#') => inside = !inside,
                (b'.', b'#', b'.') => {}
                (_, b'#', _) => {
                    if expected_edge.is_some() {
                        if Some(edge) == expected_edge {
                            inside = !inside;
                        }
                        expected_edge = None;
                    } else {
                        expected_edge = Some(match edge {
                            (b'.', b'#', b'#') => (b'#', b'#', b'.'),
                            (b'#', b'#', b'.') => (b'.', b'#', b'#'),
                            _ => panic!(),
                        });
                    }
                }
                _ => panic!(),
            }
        }
    }
    ground
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use Direction::*;

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => panic!("{value}"),
        }
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }
}

impl Direction {
    fn parse(dir_str: &str) -> Self {
        match dir_str {
            "U" => Up,
            "R" => Right,
            "D" => Down,
            "L" => Left,
            _ => panic!("Invalid instruction!"),
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Up => (-1_isize, 0_isize),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }
}

fn debug_print(ground: &[Vec<u8>], start_y: isize, start_x: isize) {
    for (y, row) in ground.iter().enumerate() {
        for (x, &symbol) in row.iter().enumerate() {
            print!(
                "{}",
                if y as isize == start_y && x as isize == start_x {
                    'X'
                } else {
                    symbol as char
                }
            );
        }
        println!();
    }
    println!();
    sleep(Duration::new(0, 100_000_000));
}
