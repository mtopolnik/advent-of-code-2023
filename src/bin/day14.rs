use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let platform: Vec<Vec<u8>> = read_to_string("input/day14.txt")
        .unwrap()
        .lines()
        .map(|str| str.as_bytes().to_vec())
        .collect();
    part1(&platform); // 106517
    let billionth = platform_after_n_cycles(1_000_000_000, platform);
    let north_beam_load = north_beam_load(&billionth);
    println!("Part 2 north beam load: {north_beam_load}"); // 79723
}

fn platform_after_n_cycles(n: usize, mut platform: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut platforms: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    for i in 1..=n {
        platform = run_cycle(platform);
        if let Some(&prev_i) = platforms.get(&platform) {
            let period_length = i - prev_i;
            return platforms
                .into_iter()
                .find(|&(_, matching_i)| matching_i == (prev_i + (n - prev_i) % period_length))
                .unwrap()
                .0;
        }
        platforms.insert(platform.clone(), i);
    }
    platform
}

fn run_cycle(mut platform: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    platform = roll_north(platform);
    platform = roll_west(platform);
    platform = roll_south(platform);
    platform = roll_east(platform);
    platform
}

fn roll_north(mut platform: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = platform.len();
    let width = platform[0].len();
    for x in 0..width {
        let mut available_y = 0;
        for y in 0..height {
            match platform[y][x] {
                b'O' => {
                    if y != available_y {
                        platform[available_y][x] = b'O';
                        platform[y][x] = b'.';
                    }
                    available_y += 1;
                }
                b'#' => {
                    available_y = y + 1;
                }
                _ => {}
            }
        }
    }
    platform
}

fn roll_west(mut platform: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = platform.len();
    let width = platform[0].len();
    for y in 0..height {
        let mut available_x = 0;
        for x in 0..width {
            match platform[y][x] {
                b'O' => {
                    if x != available_x {
                        platform[y][available_x] = b'O';
                        platform[y][x] = b'.';
                    }
                    available_x += 1;
                }
                b'#' => {
                    available_x = x + 1;
                }
                _ => {}
            }
        }
    }
    platform
}

fn roll_south(mut platform: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = platform.len();
    let width = platform[0].len();
    for x in 0..width {
        let mut available_y = height as isize - 1;
        for y in (0..height).rev() {
            match platform[y][x] {
                b'O' => {
                    if y as isize != available_y {
                        platform[available_y as usize][x] = b'O';
                        platform[y][x] = b'.';
                    }
                    available_y -= 1;
                }
                b'#' => {
                    available_y = y as isize - 1;
                }
                _ => {}
            }
        }
    }
    platform
}

fn roll_east(mut platform: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = platform.len();
    let width = platform[0].len();
    for y in 0..height {
        let mut available_x = width as isize - 1;
        for x in (0..width).rev() {
            match platform[y][x] {
                b'O' => {
                    if x as isize != available_x {
                        platform[y][available_x as usize] = b'O';
                        platform[y][x] = b'.';
                    }
                    available_x -= 1;
                }
                b'#' => {
                    available_x = x as isize - 1;
                }
                _ => {}
            }
        }
    }
    platform
}

fn north_beam_load(platform: &[Vec<u8>]) -> usize {
    let height = platform.len();
    let width = platform[0].len();
    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            if platform[y][x] == b'O' {
                result += height - y;
            }
        }
    }
    result
}

fn part1(platform: &[Vec<u8>]) {
    let height = platform.len();
    let width = platform[0].len();
    let mut result = 0;
    for x in 0..width {
        let mut available_y = 0;
        for y in 0..height {
            match platform[y][x] {
                b'O' => {
                    result += height - available_y;
                    available_y += 1;
                }
                b'#' => {
                    available_y = y + 1;
                }
                _ => {}
            }
        }
    }
    println!("Part 1 north beam load: {result}");
}
