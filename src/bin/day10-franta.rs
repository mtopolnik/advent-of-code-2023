use std::fs::read_to_string;
use std::iter::repeat;

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
}

impl From<u8> for Tile {
    fn from(symbol: u8) -> Self {
        Tile { symbol, on_path: false }
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
    let start_options = start_tile_options(&maze, start_y, start_x);
    let start_tile = &mut maze[start_y][start_x];
    start_tile.on_path = true;
    start_tile.symbol = tile_symbol(start_options);

    let (mut y, mut x) = (start_y, start_x);
    let mut direction = start_options.0;
    let mut steps = 1;
    loop {
        (y, x) = follow_direction(y, x, direction);
        let tile = &mut maze[y][x];
        if tile.on_path {
            break;
        }
        tile.on_path = true;
        steps += 1;
        direction = new_direction(tile.symbol, direction);
    }
    println!("Farthest distance: {}", steps / 2);

    let mut inner_tile_count = 0;
    for row in maze {
        let mut inside = false;
        let mut expecting_symbol = 0;
        for tile in row {
            let symbol = tile.symbol;
            if !tile.on_path {
                if inside {
                    inner_tile_count += 1;
                }
            } else {
                match symbol {
                    b'|' => {
                        inside = !inside;
                    }
                    b'-' => {}
                    b'F' => {
                        expecting_symbol = b'J';
                    }
                    b'L' => {
                        expecting_symbol = b'7';
                    }
                    b'7' | b'J' => {
                        if expecting_symbol == symbol {
                            inside = !inside;
                        }
                        expecting_symbol = 0;
                    }
                    _ => panic!("{}", symbol as char),
                }
            }
        }
    }
    println!("Inner tile count: {inner_tile_count}");
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

fn tile_symbol(available_directions: (Direction, Direction)) -> u8 {
    match available_directions {
        (Up, Down) => b'|',
        (Up, Left) => b'J',
        (Up, Right) => b'L',
        (Down, Left) => b'7',
        (Down, Right) => b'F',
        (Left, Right) => b'-',
        _ => panic!("{available_directions:?}"),
    }
}

fn start_tile_options(maze: &[Vec<Tile>], y: usize, x: usize) -> (Direction, Direction) {
    let mut options = Vec::<Direction>::new();
    if [b'|', b'7', b'F'].contains(&maze[y - 1][x].symbol) {
        options.push(Up);
    }
    if [b'|', b'J', b'L'].contains(&maze[y + 1][x].symbol) {
        options.push(Down);
    }
    if [b'-', b'F', b'L'].contains(&maze[y][x - 1].symbol) {
        options.push(Left);
    }
    if [b'-', b'J', b'7'].contains(&maze[y][x + 1].symbol) {
        options.push(Right);
    }
    (options[0], options[1])
}
