use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct DijkstraNode {
    y: u16,
    x: u16,
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

fn main() {
    let city: Vec<Vec<u8>> = read_to_string("input/day17.txt")
        .unwrap()
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();
    println!("Part 1: {}", least_heat_loss(city)); // 1263
}

fn least_heat_loss(city: Vec<Vec<u8>>) -> u32 {
    let height = city.len() as u16;
    let width = city[0].len() as u16;
    let mut unvisited: PriorityQueue<DijkstraNode, u32> = PriorityQueue::new();
    for y in 0..height {
        for x in 0..width {
            for direction in [North, East, South, West] {
                for straight_steps in [1, 2, 3] {
                    unvisited.push(
                        DijkstraNode {
                            y: y as u16,
                            x: x as u16,
                            direction: Some(direction),
                            straight_steps,
                        },
                        0,
                    );
                }
            }
        }
    }
    unvisited.push(
        DijkstraNode { y: 0, x: 0, direction: None, straight_steps: 0 },
        u32::MAX,
    );
    loop {
        let Some((dijkstra_node, priority)) = unvisited.pop() else {
            panic!("No route to the factory!");
        };
        let DijkstraNode { y, x, direction, straight_steps } = dijkstra_node;
        if priority == 0 {
            panic!("No route to the factory!")
        }
        let heat_loss = u32::MAX - priority;
        if y == height - 1 && x == width - 1 {
            return heat_loss;
        }
        let neighbors = neighbors(&city, DijkstraNode { y, x, direction, straight_steps });
        for neighbor in neighbors {
            let DijkstraNode { y: neighbor_y, x: neighbor_x, .. } = neighbor;
            let Some((_, &priority)) = unvisited.get(&neighbor) else {
                continue;
            };
            let least_heat_loss_so_far = u32::MAX - priority;
            let heat_loss_delta = city[neighbor_y as usize][neighbor_x as usize];
            let this_heat_loss = heat_loss + heat_loss_delta as u32;
            if this_heat_loss < least_heat_loss_so_far {
                unvisited.change_priority(&neighbor, u32::MAX - this_heat_loss);
            }
        }
    }
}

fn neighbors(city: &[Vec<u8>], step: DijkstraNode) -> Vec<DijkstraNode> {
    let DijkstraNode { y, x, direction, straight_steps } = step;
    let height = city.len() as i16;
    let width = city[0].len() as i16;
    let y = y as i16;
    let x = x as i16;
    let Some(incoming_direction) = direction else {
        return vec![
            DijkstraNode {
                y: 0,
                x: 1,
                direction: Some(East),
                straight_steps: 1,
            },
            DijkstraNode {
                y: 1,
                x: 0,
                direction: Some(South),
                straight_steps: 1,
            },
        ];
    };
    [Straight, Left, Right]
        .into_iter()
        .filter_map(|turn| {
            let straight_steps = if turn == Straight {
                straight_steps as i8 + 1
            } else {
                1
            };
            let outgoing_direction = incoming_direction.after_turn(turn);
            let (y, x) = follow_direction(outgoing_direction, y, x);
            (straight_steps <= 3 && y >= 0 && x >= 0 && y < height && x < width).then_some(
                DijkstraNode {
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
