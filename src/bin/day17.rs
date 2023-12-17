use memoize::memoize;
use std::{fs::read_to_string, process::exit, sync::OnceLock, thread::sleep, time::Duration};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}
use Turn::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct SearchStep {
    y: u16,
    x: u16,
    direction: Direction,
    step_countdown: u8,
    total_rotation: i8,
}

fn main() {
    static CITY: OnceLock<Vec<Vec<u8>>> = OnceLock::new();
    let city = CITY.get_or_init(|| {
        read_to_string("input/day17.txt")
            .unwrap()
            .lines()
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect()
    });
    let min_heat_loss = min_heat_loss(
        &city,
        SearchStep {
            y: 0,
            x: 0,
            direction: East,
            step_countdown: 4,
            total_rotation: 0,
        },
    );
    println!("Part 1: {min_heat_loss}");
}

#[memoize]
fn min_heat_loss(city: &'static [Vec<u8>], step1: SearchStep) -> usize {
    let min = step_options(city, step1)
        .into_iter()
        .map(|step| city[step.y as usize][step.x as usize] as usize + min_heat_loss(city, step))
        .min()
        .unwrap_or(0);
    println!("min = {min}");
    min
}

fn step_options(city: &[Vec<u8>], step: SearchStep) -> Vec<SearchStep> {
    let SearchStep { y, x, direction, step_countdown, total_rotation } = step;
    let height = city.len() as i16;
    let width = city[0].len() as i16;
    let y = y as i16;
    let x = x as i16;
    if y == height - 1 && x == width - 1 {
        return vec![];
    }
    [Straight, Left, Right]
        .into_iter()
        .filter_map(|turn| {
            let step_countdown = if turn == Straight {
                step_countdown as i8 - 1
            } else {
                3
            };
            let direction2 = direction.after_turn(turn);
            let total_rotation2 = total_rotation + i8::from(turn);
            let (y, x) = match direction2 {
                North => (y - 1, x),
                South => (y + 1, x),
                West => (y, x - 1),
                East => (y, x + 1),
            };
            if step_countdown > 0
                && total_rotation2.abs() < 4
                && y >= 0
                && x >= 0
                && y < height
                && x < width
            {
                println!(
                    "{direction:?} <{turn:?}> {direction2:?}, {total_rotation} -> {total_rotation2}"
                );
                debug_print(city, y as u16, x as u16);
                sleep(Duration::new(0, 100_000_000));
                Some(SearchStep {
                    y: y as u16,
                    x: x as u16,
                    direction: direction2,
                    step_countdown: step_countdown as u8,
                    total_rotation: total_rotation2,
                })
            } else {
                None
            }
        })
        .collect()
}

fn debug_print(city: &[Vec<u8>], y_pos: u16, x_pos: u16) {
    for y in 0..city.len() {
        for x in 0..city[0].len() {
            print!(
                "{}",
                if y as u16 == y_pos && x as u16 == x_pos {
                    'o'
                } else {
                    '.'
                }
            )
        }
        println!();
    }
    println!();
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

impl Direction {
    fn after_turn(&self, turn: Turn) -> Direction {
        let delta = i8::from(turn) + 4;
        (((u8::from(*self) + delta as u8) % 4) as u8).into()
    }
}

impl From<Turn> for i8 {
    fn from(value: Turn) -> Self {
        match value {
            Left => -1,
            Straight => 0,
            Right => 1,
        }
    }
}
