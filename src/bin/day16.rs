use std::{fs::read_to_string, thread::sleep, time::Duration};

#[derive(Default, Clone, Copy)]
struct Tile {
    symbol: u8,
    visited_directions: [bool; 4],
}

#[derive(Clone, Copy)]
struct BeamState {
    y: usize,
    x: usize,
    dir_y: isize,
    dir_x: isize,
}

fn main() {
    let mut grid: Vec<Vec<Tile>> = read_to_string("input/day16.txt")
        .unwrap()
        .lines()
        .map(|line| line.bytes().map(|b| b.into()).collect())
        .collect();
    let energized_count =
        count_energized_tiles(&mut grid, BeamState { y: 0, x: 0, dir_y: 0, dir_x: 1 });
    println!("Part 1: {energized_count}"); // 8021
    let size = grid.len();
    let max_energized_count = (0..size)
        .into_iter()
        .flat_map(|i| {
            [
                count_energized_tiles(&mut grid, BeamState { y: i, x: 0, dir_y: 0, dir_x: 1 }),
                count_energized_tiles(
                    &mut grid,
                    BeamState { y: i, x: size - 1, dir_y: 0, dir_x: -1 },
                ),
                count_energized_tiles(&mut grid, BeamState { y: 0, x: i, dir_y: 1, dir_x: 0 }),
                count_energized_tiles(
                    &mut grid,
                    BeamState { y: size - 1, x: i, dir_y: -1, dir_x: 0 },
                ),
            ]
            .into_iter()
        })
        .max()
        .unwrap();
    println!("Part 2: {max_energized_count}"); // 8216
}

fn count_energized_tiles(grid: &mut [Vec<Tile>], init_state: BeamState) -> usize {
    let size = grid.len() as isize;
    let mut todo_list: Vec<BeamState> = vec![init_state];
    loop {
        let Some(BeamState { mut y, mut x, mut dir_y, mut dir_x }) = todo_list.pop() else {
            break;
        };
        loop {
            // debug_print(grid, y, x, dir_y, dir_x);
            let tile = &mut grid[y][x];
            let did_visit = tile.did_visit_direction_mut(dir_y, dir_x);
            if *did_visit {
                break;
            }
            *did_visit = true;
            match tile.symbol {
                b'\\' => (dir_y, dir_x) = (dir_x, dir_y),
                b'/' => (dir_y, dir_x) = (-dir_x, -dir_y),
                b'-' => {
                    if dir_x == 0 {
                        add_todo_if_valid(&mut todo_list, y, x, dir_x, dir_y, size);
                        (dir_y, dir_x) = (-dir_x, -dir_y)
                    }
                }
                b'|' => {
                    if dir_y == 0 {
                        add_todo_if_valid(&mut todo_list, y, x, dir_x, dir_y, size);
                        (dir_y, dir_x) = (-dir_x, -dir_y)
                    }
                }
                b'.' => {}
                _ => panic!("Invalid tile symbol"),
            }
            let Some((new_y, new_x)) = follow_direction(y, dir_y, x, dir_x, size) else {
                break;
            };
            (y, x) = (new_y, new_x);
        }
    }
    let mut energized_count = 0;
    for row in grid.iter_mut() {
        for tile in row.iter_mut() {
            if tile.visited_directions != [false, false, false, false] {
                energized_count += 1;
                tile.visited_directions = [false, false, false, false];
            }
        }
    }
    energized_count
}

fn add_todo_if_valid(
    todo_list: &mut Vec<BeamState>,
    prev_y: usize,
    prev_x: usize,
    dir_y: isize,
    dir_x: isize,
    width: isize,
) {
    if let Some((y, x)) = follow_direction(prev_y, dir_y, prev_x, dir_x, width) {
        todo_list.push(BeamState { y, x, dir_x, dir_y });
    };
}

fn follow_direction(
    y: usize,
    dir_y: isize,
    x: usize,
    dir_x: isize,
    width: isize,
) -> Option<(usize, usize)> {
    let (new_y, new_x) = (y as isize + dir_y, x as isize + dir_x);
    (new_y >= 0 && new_x >= 0 && new_y < width && new_x < width)
        .then_some((new_y as usize, new_x as usize))
}

impl Tile {
    fn did_visit_direction_mut<'a, 'b: 'a>(
        &'b mut self,
        dir_y: isize,
        dir_x: isize,
    ) -> &'a mut bool {
        &mut self.visited_directions[(2 * ((dir_y + 1) / 2) + dir_x + 1) as usize]
    }
}

fn debug_print(grid: &[Vec<Tile>], y: usize, x: usize, dir_y: isize, dir_x: isize) {
    let dir_symbol = match (dir_y, dir_x) {
        (0, 1) => '>',
        (0, -1) => '<',
        (-1, 0) => '^',
        (1, 0) => 'v',
        _ => panic!("Invalid direction"),
    };
    for (cur_y, row) in grid.iter().enumerate() {
        for (cur_x, tile) in row.iter().enumerate() {
            print!(
                "{}",
                if (cur_y, cur_x) == (y, x) {
                    dir_symbol
                } else if tile.symbol != b'.' {
                    tile.symbol as char
                } else {
                    match tile.visited_directions {
                        [false, false, false, false] => tile.symbol as char,
                        [true, false, false, false] => '<',
                        [false, true, false, false] => '^',
                        [false, false, true, false] => '>',
                        [false, false, false, true] => 'v',
                        _ => '+',
                    }
                }
            )
        }
        println!();
    }
    println!();
    sleep(Duration::new(0, 10_000_000));
}

impl From<u8> for Tile {
    fn from(symbol: u8) -> Self {
        Tile { symbol, ..Default::default() }
    }
}
