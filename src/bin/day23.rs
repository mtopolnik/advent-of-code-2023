use std::{cmp::max, collections::VecDeque, fs::read_to_string};
use Direction::*;
use Turn::*;

fn main() {
    let input = read_to_string("input/day23.txt").unwrap();
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let (start_d, (start_y, start_x)) = (South, (0_usize, 1_usize));
    let (end_y, end_x) = (grid.len() - 1, grid[0].len() - 2);

    let mut longest_path_len = 0;
    let mut todo = VecDeque::new();
    todo.push_back((0, start_d, (start_y, start_x)));
    loop {
        let Some((path_len, d, (y, x))) = todo.pop_front() else {
            break;
        };
        if (y, x) == (end_y, end_x) {
            longest_path_len = max(longest_path_len, path_len);
        }
        let neighbors = all_neighbors(&grid, y, x, d);
        for (next_d, next_coords) in neighbors {
            todo.push_back((path_len + 1, next_d, next_coords));
        }
    }
    println!("Part 1: {longest_path_len}"); // 2298
}

fn all_neighbors(
    grid: &[Vec<u8>],
    y: usize,
    x: usize,
    d: Direction,
) -> VecDeque<(Direction, (usize, usize))> {
    [Straight, Left, Right]
        .into_iter()
        .map(|turn| d.after_turn(turn))
        .map(|d| (d, follow_direction(d, y, x)))
        .filter(|(d, (y, x))| {
            (0..grid.len()).contains(y)
                && (0..grid[0].len()).contains(x)
                && [b'.', d.to_ascii()].contains(&grid[*y][*x])
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    South,
    West,
    North,
    East,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Direction {
    fn after_turn(&self, turn: Turn) -> Direction {
        let delta = 4 + match turn {
            Left => -1,
            Straight => 0,
            Right => 1,
        };
        (((u8::from(*self) + delta as u8) % 4) as u8).into()
    }

    fn to_ascii(&self) -> u8 {
        match self {
            South => b'v',
            West => b'<',
            North => b'^',
            East => b'>',
        }
    }
}

fn follow_direction(direction: Direction, y: usize, x: usize) -> (usize, usize) {
    match direction {
        North => (y - 1, x),
        South => (y + 1, x),
        West => (y, x - 1),
        East => (y, x + 1),
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!("{value}"),
        }
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }
}
