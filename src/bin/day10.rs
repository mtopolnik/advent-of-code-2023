use std::fs::read_to_string;
use std::iter::repeat;
use std::ops::Range;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

#[derive(Copy, Clone, Debug)]
struct Tile {
    symbol: u8,
    on_path: bool,
    inside: bool,
}

impl From<u8> for Tile {
    fn from(symbol: u8) -> Self {
        Tile { symbol, on_path: false, inside: false }
    }
}

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day10.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let empty_line = repeat(".")
        .take(input_lines[0].len() + 2)
        .collect::<String>();
    let mut lines = vec![empty_line.clone()];
    for line in input_lines {
        lines.push(format!(".{}.", line));
    }
    lines.push(empty_line);

    let mut maze: Vec<Vec<Tile>> = lines
        .into_iter()
        .map(|s| s.bytes().map(|b| b.into()).collect())
        .collect();

    let (start_y, start_x) = find_start(&maze);
    maze[start_y][start_x].on_path = true;
    let start_direction = {
        if [b'|', b'7', b'F'].contains(&maze[start_y - 1][start_x].symbol) {
            Up
        } else if [b'-', b'L', b'F'].contains(&maze[start_y][start_x - 1].symbol) {
            Left
        } else if [b'-', b'J', b'7'].contains(&maze[start_y][start_x + 1].symbol) {
            Right
        } else if [b'|', b'J', b'L'].contains(&maze[start_y + 1][start_x].symbol) {
            Down
        } else {
            panic!("dead end");
        }
    };

    let (mut y, mut x) = (start_y, start_x);
    let mut steps = 1;
    let mut net_rotation = 0;
    let mut direction = start_direction;
    loop {
        (y, x) = follow_direction(y, x, direction);
        let tile = &mut maze[y][x];
        if tile.symbol == b'S' {
            break;
        }
        tile.on_path = true;
        steps += 1;

        let prev_direction = direction;
        direction = new_direction(tile.symbol, direction);
        net_rotation += get_rotation(prev_direction, direction);
    }
    println!(
        "Farthest distance: {}, net rotation: {net_rotation}",
        steps / 2
    );

    (y, x) = (start_y, start_x);
    direction = start_direction;
    loop {
        (y, x) = follow_direction(y, x, direction);
        let on_symbol = maze[y][x].symbol;
        if on_symbol == b'S' {
            break;
        }
        match (on_symbol, direction) {
            (b'|', Up) => mark_left(y, x, &mut maze),
            (b'|', Down) => mark_right(y, x, &mut maze),
            (b'-', Left) => mark_down(y, x, &mut maze),
            (b'-', Right) => mark_up(y, x, &mut maze),
            (b'7', Up) => {
                mark_left(y, x, &mut maze);
                mark_down(y, x, &mut maze);
            }
            (b'7', Right) => {
                mark_up(y, x, &mut maze);
                mark_right(y, x, &mut maze);
            }
            (b'F', Up) => {
                mark_left(y, x, &mut maze);
                mark_up(y, x, &mut maze);
            }
            (b'F', Left) => {
                mark_down(y, x, &mut maze);
                mark_right(y, x, &mut maze);
            }
            (b'J', Down) => {
                mark_right(y, x, &mut maze);
                mark_down(y, x, &mut maze);
            }
            (b'J', Right) => {
                mark_up(y, x, &mut maze);
                mark_left(y, x, &mut maze);
            }
            (b'L', Down) => {
                mark_right(y, x, &mut maze);
                mark_up(y, x, &mut maze);
            }
            (b'L', Left) => {
                mark_down(y, x, &mut maze);
                mark_left(y, x, &mut maze);
            }
            _ => {}
        }
        direction = new_direction(on_symbol, direction);
    }

    for row in maze.iter() {
        println!(
            "{}",
            row.iter()
                .map(|&tile| {
                    if tile.on_path {
                        'o'
                    } else if tile.inside {
                        'i'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
    }
    let inner_tile_count = maze
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|tile| tile.inside)
        .count();
    println!("Inner tile count: {inner_tile_count}");
}

fn mark_horizontal(y: usize, x_range: Range<usize>, maze: &mut [Vec<Tile>]) {
    for inner_x in x_range {
        let inner_tile = &mut maze[y][inner_x];
        if inner_tile.on_path {
            break;
        }
        inner_tile.inside = true;
    }
}

fn mark_vertical(y_range: Range<usize>, x: usize, maze: &mut [Vec<Tile>]) {
    for inner_y in y_range {
        let inner_tile = &mut maze[inner_y][x];
        if inner_tile.on_path {
            break;
        }
        inner_tile.inside = true;
    }
}

fn mark_left(y: usize, x: usize, maze: &mut [Vec<Tile>]) {
    mark_horizontal(y, x - 1..0, maze);
}

fn mark_right(y: usize, x: usize, maze: &mut [Vec<Tile>]) {
    mark_horizontal(y, x + 1..maze[y].len() - 1, maze);
}

fn mark_down(y: usize, x: usize, maze: &mut [Vec<Tile>]) {
    mark_vertical(y + 1..maze.len() - 1, x, maze);
}

fn mark_up(y: usize, x: usize, maze: &mut [Vec<Tile>]) {
    mark_vertical(y - 1..0, x, maze);
}

fn get_rotation(prev_direction: Direction, new_direction: Direction) -> i8 {
    match (prev_direction, new_direction) {
        (prev, new) if prev == new => 0,
        (Up, Left) => 1,
        (Left, Down) => 1,
        (Down, Right) => 1,
        (Right, Up) => 1,
        _ => -1,
    }
}

fn new_direction(symbol: u8, direction: Direction) -> Direction {
    match (symbol, direction) {
        (b'|', Up) => Up,
        (b'|', Down) => Down,
        (b'-', Left) => Left,
        (b'-', Right) => Right,
        (b'7', Up) => Left,
        (b'7', Right) => Down,
        (b'F', Up) => Right,
        (b'F', Left) => Down,
        (b'J', Down) => Left,
        (b'J', Right) => Up,
        (b'L', Down) => Right,
        (b'L', Left) => Up,
        _ => panic!("{}, {direction:?}", symbol as char),
    }
}

fn follow_direction(y: usize, x: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Up => (y - 1, x),
        Down => (y + 1, x),
        Left => (y, x - 1),
        Right => (y, x + 1),
    }
}

fn find_start(maze: &[Vec<Tile>]) -> (usize, usize) {
    maze.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &tile)| tile.symbol == b'S')
                .map(|(x, _)| (y, x))
        })
        .filter_map(|option| option)
        .next()
        .unwrap()
}
