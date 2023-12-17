use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::read_to_string,
    sync::OnceLock,
    thread::sleep,
    time::Duration,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Start,
    Up,
    Down,
    Left,
    Right,
}
use memoize::memoize;
use Direction::*;

struct StepOption {
    direction: Direction,
    step_countdown: u8,
    y: u16,
    x: u16,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            Start => Start,
        }
    }
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
    let min_heat_loss = min_heat_loss(&city, 0, 0, Start, 0, Vec::new());
    println!("Part 1: {min_heat_loss}");
}

#[memoize]
fn min_heat_loss(
    city: &'static [Vec<u8>],
    y: u16,
    x: u16,
    direction: Direction,
    step_countdown: u8,
    mut visited: Vec<(u16, u16)>,
) -> usize {
    if visited.contains(&(y, x)) {
        return usize::MAX;
    }
    visited.push((y, x));
    debug_print(&city, &visited);
    step_options(city, y, x, direction, step_countdown)
        .into_iter()
        .map(|StepOption { direction, step_countdown, y, x }| {
            let min_heat_loss =
                min_heat_loss(city, y, x, direction, step_countdown, visited.clone());
            min_heat_loss
                + if min_heat_loss == usize::MAX {
                    0
                } else {
                    city[y as usize][x as usize] as usize
                }
        })
        .min()
        .unwrap_or(0)
}

fn step_options(
    city: &[Vec<u8>],
    y: u16,
    x: u16,
    direction: Direction,
    step_countdown: u8,
) -> Vec<StepOption> {
    let height = city.len() as i16;
    let width = city[0].len() as i16;
    let y = y as i16;
    let x = x as i16;
    if y == height - 1 && x == width - 1 {
        println!("Reached factory");
        return vec![];
    }
    [Right, Down, Left, Up]
        .into_iter()
        .filter(|&d| d != direction.opposite())
        .filter_map(|d| {
            let step_countdown = if d == direction {
                step_countdown as i8 - 1
            } else {
                2
            };
            let (y, x) = match d {
                Up => (y - 1, x),
                Down => (y + 1, x),
                Left => (y, x - 1),
                Right => (y, x + 1),
                _ => panic!("What??"),
            };
            (step_countdown >= 0 && y >= 0 && x >= 0 && y < height && x < width).then_some(
                StepOption {
                    direction: d,
                    step_countdown: step_countdown as u8,
                    y: y as u16,
                    x: x as u16,
                },
            )
        })
        .collect()
}

fn debug_print(city: &[Vec<u8>], path: &[(u16, u16)]) {
    let height = city.len() as u16;
    let width = city[0].len() as u16;
    let path_set: HashSet<(u16, u16), RandomState> = HashSet::from_iter(path.iter().cloned());
    for y in 0..height {
        for x in 0..width {
            print!("{}", if path_set.contains(&(y, x)) { 'o' } else { ' ' });
        }
        println!();
    }
    println!();
    sleep(Duration::new(0, 100_000_000));
}
