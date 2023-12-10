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

    let maze: Vec<Vec<u8>> = lines.into_iter().map(|s| s.as_bytes().to_owned()).collect();

    let (mut y, mut x) = find_start(&maze);
    let mut direction;

    direction = {
        if [b'|', b'7', b'F'].contains(&maze[y - 1][x]) {
            Up
        } else if [b'|', b'J', b'L'].contains(&maze[y + 1][x]) {
            Down
        } else if [b'-', b'L', b'F'].contains(&maze[y][x - 1]) {
            Left
        } else if [b'-', b'J', b'7'].contains(&maze[y][x + 1]) {
            Right
        } else {
            panic!("dead end");
        }
    };
    (y, x) = follow_direction(y, x, direction);

    let mut steps = 1;
    loop {
        let on_ch = maze[y][x];
        if on_ch == b'S' {
            break;
        }
        steps += 1;
        direction = {
            match (on_ch, direction) {
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
                _ => panic!("{}, {direction:?}", on_ch as char),
            }
        };
        (y, x) = follow_direction(y, x, direction);
    }
    println!("Farthest distance: {}", steps / 2);
}

fn follow_direction(y: usize, x: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Up => (y - 1, x),
        Down => (y + 1, x),
        Left => (y, x - 1),
        Right => (y, x + 1),
    }
}

fn find_start(maze: &[Vec<u8>]) -> (usize, usize) {
    maze.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &ch)| ch == b'S')
                .map(|(x, _)| (y, x))
        })
        .filter_map(|option| option)
        .next()
        .unwrap()
}
