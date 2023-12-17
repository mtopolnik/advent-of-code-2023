use std::{fs::read_to_string, thread::sleep, time::Duration};

#[derive(Default, Clone, Copy)]
struct Tile {
    symbol: u8,
    visited_directions: [bool; 4],
}

#[derive(Clone, Copy)]
struct Photon {
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
        count_energized_tiles(&mut grid, Photon { y: 0, x: 0, dir_y: 0, dir_x: 1 });
    println!("Part 1: {energized_count}"); // 8021
    let size = grid.len();
    let max_energized_count = (0..size)
        .into_iter()
        .flat_map(|i| {
            [
                count_energized_tiles(&mut grid, Photon { y: i, x: 0, dir_y: 0, dir_x: 1 }),
                count_energized_tiles(&mut grid, Photon { y: i, x: size - 1, dir_y: 0, dir_x: -1 }),
                count_energized_tiles(&mut grid, Photon { y: 0, x: i, dir_y: 1, dir_x: 0 }),
                count_energized_tiles(&mut grid, Photon { y: size - 1, x: i, dir_y: -1, dir_x: 0 }),
            ]
            .into_iter()
        })
        .max()
        .unwrap();
    println!("Part 2: {max_energized_count}"); // 8216
}

fn count_energized_tiles(grid: &mut [Vec<Tile>], init_state: Photon) -> usize {
    let size = grid.len() as isize;
    let mut photons: Vec<Photon> = vec![init_state];
    while !photons.is_empty() {
        // debug_print(grid);
        let mut i = 0;
        while i < photons.len() {
            let Photon { y, x, mut dir_y, mut dir_x } = photons[i];
            let tile = &mut grid[y][x];
            let did_visit = tile.did_visit_direction_mut(dir_y, dir_x);
            if *did_visit {
                photons.remove(i);
                break;
            }
            *did_visit = true;
            match tile.symbol {
                b'\\' => (dir_y, dir_x) = (dir_x, dir_y),
                b'/' => (dir_y, dir_x) = (-dir_x, -dir_y),
                b'-' => {
                    if dir_x == 0 {
                        photons.push(Photon { y, x, dir_y: dir_x, dir_x: dir_y });
                        (dir_y, dir_x) = (-dir_x, -dir_y)
                    }
                }
                b'|' => {
                    if dir_y == 0 {
                        photons.push(Photon { y, x, dir_y: dir_x, dir_x: dir_y });
                        (dir_y, dir_x) = (-dir_x, -dir_y)
                    }
                }
                b'.' => {}
                _ => panic!("Invalid tile symbol"),
            }
            let (y, x) = (y as isize + dir_y, x as isize + dir_x);
            if y < 0 || x < 0 || y >= size || x >= size {
                photons.remove(i);
            } else {
                photons[i] = Photon { y: y as usize, x: x as usize, dir_y, dir_x };
                i += 1;
            }
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

impl Tile {
    fn did_visit_direction_mut<'a, 'b: 'a>(
        &'b mut self,
        dir_y: isize,
        dir_x: isize,
    ) -> &'a mut bool {
        &mut self.visited_directions[(2 * ((dir_y + 1) / 2) + dir_x + 1) as usize]
    }
}

fn debug_print(grid: &[Vec<Tile>]) {
    for row in grid.iter() {
        for tile in row.iter() {
            print!(
                "{}",
                if tile.symbol != b'.' {
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
    sleep(Duration::new(0, 4_000_000));
}

impl From<u8> for Tile {
    fn from(symbol: u8) -> Self {
        Tile { symbol, ..Default::default() }
    }
}
