use std::{collections::HashMap, fs::read_to_string, thread::sleep, time::Duration};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct Block {
    weight: u8,
    direction: Option<Direction>,
    straight_steps: u8,
}

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
use priority_queue::PriorityQueue;
use Turn::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct SearchStep {
    y: u16,
    x: u16,
    direction: Option<Direction>,
    straight_steps: u8,
}

fn main() {
    let city: Vec<Vec<Block>> = read_to_string("input/day17.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| Block { weight: b - b'0', ..Default::default() })
                .collect()
        })
        .collect();
    println!("Part 1: {}", least_heat_loss(city)); // 1239 is too low
}

fn least_heat_loss(mut city: Vec<Vec<Block>>) -> u32 {
    let height = city.len() as u16;
    let width = city[0].len() as u16;
    let mut unvisited: PriorityQueue<(u16, u16), u32> = PriorityQueue::new();
    for y in 0..height {
        for x in 0..width {
            unvisited.push(
                (y as u16, x as u16),
                if y == 0 && x == 0 { u32::MAX } else { 0 },
            );
        }
    }
    loop {
        let Some(((y, x), priority)) = unvisited.pop() else {
            panic!("No route to the factory!");
        };
        if priority == 0 {
            panic!("No route to the factory!")
        }
        debug_print_route(y as i16, x as i16, &city);
        let heat_loss = u32::MAX - priority;
        if y == height - 1 && x == width - 1 {
            return heat_loss;
        }
        let visiting = city[y as usize][x as usize];
        let step_options = step_options(
            &city,
            SearchStep {
                y,
                x,
                direction: visiting.direction,
                straight_steps: visiting.straight_steps,
            },
        );
        for SearchStep {
            y: next_y,
            x: next_x,
            direction: next_direction,
            straight_steps,
        } in step_options
        {
            let Some((_, &priority)) = unvisited.get(&(next_y, next_x)) else {
                continue;
            };
            let least_heat_loss = u32::MAX - priority;
            let next_block = &mut city[next_y as usize][next_x as usize];
            let next_weight = next_block.weight as u32;
            let next_heat_loss = heat_loss + next_weight;
            if next_heat_loss < least_heat_loss {
                unvisited.change_priority(&(next_y, next_x), u32::MAX - next_heat_loss);
                next_block.direction = next_direction;
                next_block.straight_steps = straight_steps;
            }
        }
    }
}

fn step_options(city: &[Vec<Block>], step: SearchStep) -> Vec<SearchStep> {
    let SearchStep { y, x, direction, straight_steps } = step;
    let height = city.len() as i16;
    let width = city[0].len() as i16;
    let y = y as i16;
    let x = x as i16;
    let Some(incoming_direction) = direction else {
        return vec![
            SearchStep {
                y: 0,
                x: 1,
                direction: Some(East),
                straight_steps: 0,
            },
            SearchStep {
                y: 1,
                x: 0,
                direction: Some(South),
                straight_steps: 0,
            },
        ];
    };
    [Straight, Left, Right]
        .into_iter()
        .filter_map(|turn| {
            let straight_steps = if turn == Straight {
                straight_steps as i8 + 1
            } else {
                0
            };
            let outgoing_direction = incoming_direction.after_turn(turn);
            let (y, x) = follow_direction(outgoing_direction, y, x);
            (straight_steps < 3 && y >= 0 && x >= 0 && y < height && x < width).then_some(
                SearchStep {
                    y: y as u16,
                    x: x as u16,
                    direction: Some(outgoing_direction),
                    straight_steps: straight_steps as u8,
                },
            )
        })
        .collect()
}

fn follow_direction(direction: Direction, y: i16, x: i16) -> (i16, i16) {
    match direction {
        North => (y - 1, x),
        South => (y + 1, x),
        West => (y, x - 1),
        East => (y, x + 1),
    }
}

fn debug_print_route(mut y: i16, mut x: i16, city: &[Vec<Block>]) {
    let mut directions = HashMap::new();
    loop {
        if (y, x) == (0, 0) {
            break;
        }
        let (delta_y, delta_x, symbol) = match city[y as usize][x as usize].direction.unwrap() {
            East => (0, 1, '>'),
            West => (0, -1, '<'),
            South => (1, 0, 'v'),
            North => (-1, 0, '^'),
        };
        directions.insert((y, x), symbol);
        (y, x) = (y - delta_y, x - delta_x);
    }
    for y in 0..city.len() {
        for x in 0..city[0].len() {
            if let Some(symbol) = directions.get(&(y as i16, x as i16)) {
                print!("{symbol}")
            } else {
                print!("{}", city[y][x].weight);
            };
        }
        println!();
    }
    println!();
    sleep(Duration::new(0, 20_000_000));
}

fn debug_print(city: &[Vec<Block>]) {
    for y in 0..city.len() {
        for x in 0..city[0].len() {
            let block = city[y][x];
            print!("{}", if block.direction.is_some() { 'o' } else { '.' })
        }
        println!();
    }
    println!();
    sleep(Duration::new(0, 100_000_000));
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
