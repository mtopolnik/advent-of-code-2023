use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let input = &read_to_string("input/day18.txt").unwrap();
    let (instructions_1, instructions_2): (Vec<(Direction, usize)>, Vec<(Direction, usize)>) =
        input
            .lines()
            .map(|line| {
                let [dir_str, count_str, code] = line.split(" ").collect::<Vec<_>>()[..] else {
                    panic!("Parse error");
                };
                (
                    (
                        Direction::parse_part1(dir_str),
                        str::parse(count_str).unwrap(),
                    ),
                    (
                        Direction::parse_part2(code.chars().nth(7).unwrap()),
                        usize::from_str_radix(&code[2..7], 16).unwrap(),
                    ),
                )
            })
            .unzip();
    let ground = build_trench(&instructions_1);
    let count_part1 = dugout_block_count(ground);
    println!("Part 1: {count_part1}\n\n");

    let ground = build_trench(&instructions_2);
    let count_part2 = dugout_block_count(ground);
    println!("Part 2: {count_part2}"); // 42708339569950
}

fn build_trench(instructions: &[(Direction, usize)]) -> VecDeque<Vec<u32>> {
    let start_x = 2_000_000_u32;
    let mut ground: VecDeque<Vec<u32>> = VecDeque::new();
    ground.push_back(vec![]);
    ground.push_back(vec![start_x]);
    ground.push_back(vec![]);
    let (mut y, mut x) = (1_isize, start_x as isize);
    for &(dir, count) in instructions {
        let (delta_y, delta_x) = dir.delta();
        for _ in 0..count {
            (y, x) = (y + delta_y, x + delta_x);
            if x == 0 {
                panic!("Reached left edge of ground!");
            }
            ground[y as usize].push(x as u32);
            if y == 0 {
                ground.push_front(vec![]);
                y = 1;
            } else if y == ground.len() as isize - 1 {
                ground.push_back(vec![]);
            }
        }
    }
    for row in ground.iter_mut() {
        row.sort();
    }
    ground
}

fn dugout_block_count(ground: VecDeque<Vec<u32>>) -> usize {
    let mut dugout_count = 0_usize;
    let print_skip = ground.len() / 20;
    for y in 1..ground.len() - 1 {
        if y % print_skip == 0 {
            println!("{:.1}% complete", 100.0 * y as f32 / ground.len() as f32);
        }
        let row = &ground[y];
        let mut prev_trench_x = -1_i32;
        let mut inside = false;
        let mut expected_edge: Option<(u8, u8, u8)> = None;
        for trench_x in row {
            if *trench_x as i32 <= prev_trench_x {
                println!("{trench_x} <= {prev_trench_x}");
                continue;
            }
            dugout_count += 1;
            if inside {
                if prev_trench_x < 0 {
                    println!("prev_trench_x < 0: {prev_trench_x}");
                }
                dugout_count += (trench_x - (prev_trench_x as u32) - 1) as usize;
            }
            let symbol_above = match ground[y - 1].contains(trench_x) {
                true => b'#',
                false => b'.',
            };
            let symbol_below = match ground[y + 1].contains(trench_x) {
                true => b'#',
                false => b'.',
            };
            let edge = (symbol_above, b'#', symbol_below);
            match edge {
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
            prev_trench_x = *trench_x as i32;
        }
    }
    dugout_count
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
    fn parse_part1(dir_str: &str) -> Self {
        match dir_str {
            "R" => Right,
            "D" => Down,
            "L" => Left,
            "U" => Up,
            _ => panic!("Invalid instruction!"),
        }
    }

    fn parse_part2(dir_ch: char) -> Self {
        match dir_ch {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => panic!("{dir_ch}"),
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
            Up => (-1_isize, 0_isize),
        }
    }
}
