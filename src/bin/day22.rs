use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use regex::Regex;

const X_SIZE: usize = 10;
const Y_SIZE: usize = 10;
const Z_SIZE: usize = 375;

type Grid = [[[u16; Z_SIZE]; Y_SIZE]; X_SIZE];

fn main() {
    let input = read_to_string("input/day22.txt").unwrap();
    let mut grid: Grid = [[[0_u16; Z_SIZE]; Y_SIZE]; X_SIZE];
    let block_re = Regex::new(r"(\d),(\d),(\d+)~(\d),(\d),(\d+)").unwrap();
    for (i, line) in input.lines().enumerate() {
        let [start_x, start_y, start_z, end_x, end_y, end_z, ..] = *block_re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .filter_map(|x| x)
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        else {
            panic!("Parse error");
        };
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                for z in start_z..=end_z {
                    grid[x][y][z] = (i + 1) as u16;
                }
            }
        }
    }
    let_bricks_fall(&mut grid);
    let (supporting_brick_counts, supported_bricks) = analyze_dependencies(&grid);
    let removable_bricks = find_removable_bricks(&grid);
    println!("Part 1: {}", removable_bricks.len()); // 482
    let cascading_brick_total_count =
        count_all_cascading_bricks(supporting_brick_counts, supported_bricks);
    println!("Part 2: {cascading_brick_total_count}"); // 103010
}

fn let_bricks_fall(grid: &mut Grid) {
    let mut fallen_bricks = [false; 1494];
    for z in 1..Z_SIZE {
        for x in 0..X_SIZE {
            for y in 0..Y_SIZE {
                let brick_id = grid[x][y][z];
                if brick_id == 0 || fallen_bricks[brick_id as usize] {
                    continue;
                }
                fallen_bricks[brick_id as usize] = true;
                if let Some(vertical_brick_top) = (z + 1..Z_SIZE)
                    .filter(|&z_prime| grid[x][y][z_prime] == brick_id)
                    .last()
                {
                    // move vertical brick
                    let fall_distance = fall_distance(&grid, x, y, z);
                    if fall_distance > 0 {
                        let column = &mut grid[x][y];
                        for brick_z in z..=vertical_brick_top {
                            column[brick_z] = 0_u16;
                            column[brick_z - fall_distance] = brick_id;
                        }
                    }
                } else {
                    // move horizontal brick
                    let brick_coords = all_brick_coords(&grid, x, y, z);
                    let fall_distance = brick_coords
                        .iter()
                        .map(|&(brick_x, brick_y)| fall_distance(&grid, brick_x, brick_y, z))
                        .min()
                        .unwrap();
                    if fall_distance > 0 {
                        for (brick_x, brick_y) in brick_coords {
                            grid[brick_x][brick_y][z] = 0_u16;
                            grid[brick_x][brick_y][z - fall_distance] = brick_id;
                        }
                    }
                }
            }
        }
    }
}

fn find_removable_bricks(grid: &Grid) -> HashSet<u16> {
    let mut removable_bricks = HashSet::<u16>::new();
    for z in (1..Z_SIZE).rev() {
        for x in 0..X_SIZE {
            for y in 0..Y_SIZE {
                let brick_id = grid[x][y][z];
                if brick_id == 0 || removable_bricks.contains(&brick_id) {
                    continue;
                }
                if z == Z_SIZE - 1 {
                    // everything on the top cascades zero bricks
                    removable_bricks.insert(brick_id);
                    continue;
                }
                if grid[x][y][z + 1] == brick_id {
                    // we're on the middle of a vertical brick, so we already handled it
                    continue;
                }
                let coords_with_brick_above = all_brick_coords(&grid, x, y, z)
                    .into_iter()
                    .filter(|&(brick_x, brick_y)| grid[brick_x][brick_y][z + 1] != 0)
                    .collect::<Vec<_>>();
                if coords_with_brick_above.is_empty() {
                    removable_bricks.insert(brick_id);
                    continue;
                }
                let mut supported_ids = coords_with_brick_above
                    .iter()
                    .map(|&(x_above, y_above)| grid[x_above][y_above][z + 1])
                    .collect::<BTreeSet<_>>();
                for (brick_x, brick_y) in coords_with_brick_above {
                    let brick_above_coords = all_brick_coords(&grid, brick_x, brick_y, z + 1);
                    let has_other_support = brick_above_coords
                        .into_iter()
                        .filter(|&(x_prime, y_prime)| {
                            ![0, brick_id].contains(&grid[x_prime][y_prime][z])
                        })
                        .next()
                        .is_some();
                    if has_other_support {
                        supported_ids.remove(&grid[brick_x][brick_y][z + 1]);
                    }
                }
                if supported_ids.is_empty() {
                    removable_bricks.insert(brick_id);
                }
            }
        }
    }
    removable_bricks
}

fn analyze_dependencies(grid: &Grid) -> (HashMap<u16, u16>, HashMap<u16, HashSet<u16>>) {
    // brick_id -> how many bricks support it
    let mut supporting_brick_counts = HashMap::<u16, u16>::new();
    // brick_id -> which bricks it supports
    let mut supported_bricks = HashMap::<u16, HashSet<u16>>::new();
    for z in 1..Z_SIZE {
        for x in 0..X_SIZE {
            for y in 0..Y_SIZE {
                let brick_id = grid[x][y][z];
                if brick_id == 0 {
                    continue;
                }
                if z == 1 {
                    // everything on the bottom is supported by zero bricks
                    supporting_brick_counts.insert(brick_id, 0);
                }
                if z == Z_SIZE - 1 {
                    // everything on the top supports zero bricks
                    supported_bricks.insert(brick_id, HashSet::new());
                    continue;
                }
                let brick_id_above = grid[x][y][z + 1];
                if brick_id_above == brick_id {
                    // This is a part of a vertical brick; we'll get back to it on the next row
                    continue;
                }
                if brick_id_above == 0 {
                    supported_bricks.entry(brick_id).or_default();
                    continue;
                }
                let is_new_supported_brick = supported_bricks
                    .entry(brick_id)
                    .or_default()
                    .insert(brick_id_above);
                if is_new_supported_brick {
                    *supporting_brick_counts.entry(brick_id_above).or_default() += 1;
                }
            }
        }
    }
    let supporting_brick_counts = supporting_brick_counts;
    let supported_bricks = supported_bricks;
    (supporting_brick_counts, supported_bricks)
}

fn count_all_cascading_bricks(
    supporting_brick_counts: HashMap<u16, u16>,
    supported_bricks: HashMap<u16, HashSet<u16>>,
) -> usize {
    let mut cascading_count_total = 0;
    for brick_id in supported_bricks.keys() {
        let mut local_supporting_counts = HashMap::<u16, u16>::new();
        let mut cascading_bricks = VecDeque::<u16>::new();
        cascading_bricks.push_back(*brick_id);
        loop {
            let Some(falling_id) = cascading_bricks.pop_front() else {
                break;
            };
            for supported_id in supported_bricks.get(&falling_id).unwrap() {
                let supporting_count = local_supporting_counts
                    .entry(*supported_id)
                    .or_insert_with(|| supporting_brick_counts[supported_id]);
                *supporting_count -= 1;
                if *supporting_count == 0 {
                    cascading_bricks.push_back(*supported_id);
                    cascading_count_total += 1;
                }
            }
        }
    }
    cascading_count_total
}

fn all_brick_coords(grid: &Grid, x: usize, y: usize, z: usize) -> Vec<(usize, usize)> {
    let brick_id = grid[x][y][z];
    let x_aligned = (0..X_SIZE)
        .filter(|&x_prime| grid[x_prime][y][z] == brick_id)
        .map(|brick_x| (brick_x, y))
        .collect::<Vec<_>>();
    if x_aligned.len() > 1 {
        return x_aligned;
    }
    (0..Y_SIZE)
        .filter(|&y_prime| grid[x][y_prime][z] == brick_id)
        .map(|brick_y| (x, brick_y))
        .collect::<Vec<_>>()
}

fn fall_distance(grid: &Grid, x: usize, y: usize, z: usize) -> usize {
    let column = grid[x][y];
    let bottom_z = (0..z)
        .rev()
        .filter(|&z_prime| z_prime == 0 || column[z_prime] != 0)
        .next()
        .unwrap();
    z - bottom_z - 1
}
