use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};
use Direction::*;
use Turn::*;

type Tile = (u8, u8);
type SearchState = (Tile, Direction);

fn main() {
    let input = read_to_string("input/day23.txt").unwrap();
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let enter_tile = (0_u8, 1_u8);
    let exit_tile = (grid.len() as u8 - 1, grid[0].len() as u8 - 2);

    let mut max_distance = 0_usize;
    let incidence_map = crossroads_graph(grid, enter_tile, exit_tile);
    let mut todo = Vec::<(Tile, HashSet<Tile>, usize)>::new();
    todo.push((enter_tile, HashSet::new(), 0));
    loop {
        let Some((tile, visited_tiles, traveled_distance)) = todo.pop() else {
            break;
        };
        if tile == exit_tile {
            if traveled_distance > max_distance {
                max_distance = traveled_distance;
            }
            continue;
        }
        if visited_tiles.contains(&tile) {
            continue;
        }
        let Some(neighs) = incidence_map.get(&tile) else {
            panic!("{tile:?} not found");
        };
        for &(neigh_tile, distance) in neighs {
            let mut visited_tiles = visited_tiles.clone();
            visited_tiles.insert(tile);
            todo.push((
                neigh_tile,
                visited_tiles,
                traveled_distance + distance as usize,
            ));
        }
    }
    println!("max distance: {max_distance}");
}

fn crossroads_graph(
    grid: Vec<Vec<u8>>,
    enter_tile: Tile,
    exit_tile: Tile,
) -> HashMap<Tile, Vec<(Tile, usize)>> {
    let mut incidence_map = HashMap::<Tile, Vec<(Tile, usize)>>::new();
    let mut visited_crossroads = HashSet::<Tile>::new();
    let mut todo = Vec::<SearchState>::new();
    todo.push((enter_tile, South));
    loop {
        let Some(start_state) = todo.pop() else {
            break;
        };
        let (start_tile, mut direction) = start_state;
        let mut curr_tile = follow_direction(start_state);
        let mut distance = 1;

        // find the next node (crossing or exit) and connect it to (start_y, start_x)
        loop {
            if curr_tile == exit_tile {
                incidence_map
                    .entry(start_tile)
                    .or_default()
                    .push((curr_tile, distance));
                break;
            }
            let next_ds = available_directions(&grid, (curr_tile, direction));
            if next_ds.is_empty() {
                break;
            }
            if next_ds.len() == 1 {
                // we're in a corridor, follow it
                direction = next_ds.into_iter().next().unwrap();
                curr_tile = follow_direction((curr_tile, direction));
                distance += 1;
                continue;
            }
            // next_ds.len() > 1 -- we're on a crossroads
            if start_tile != enter_tile {
                incidence_map
                    .entry(curr_tile)
                    .or_default()
                    .push((start_tile, distance));
            }
            if !visited_crossroads.contains(&curr_tile) {
                incidence_map
                    .entry(start_tile)
                    .or_default()
                    .push((curr_tile, distance));
                visited_crossroads.insert(curr_tile);
                for next_d in next_ds {
                    todo.push((curr_tile, next_d));
                }
            }
            break;
        }
    }
    incidence_map
}

fn available_directions(grid: &[Vec<u8>], state: SearchState) -> Vec<Direction> {
    let ((y, x), d) = state;
    [Right, Left, Straight]
        .into_iter()
        .map(|turn| d.after_turn(turn))
        .filter(|&d| {
            let (y, x) = follow_direction(((y, x), d));
            grid[y as usize][x as usize] != b'#'
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    South,
    West,
    North,
    East,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Direction {
    fn after_turn(&self, turn: Turn) -> Direction {
        let delta = 4 + match turn {
            Left => -1,
            Straight => 0,
            Right => 1,
        };
        (((u8::from(*self) + delta as u8) % 4) as u8).into()
    }
}

fn follow_direction(search_state: SearchState) -> Tile {
    let ((y, x), d) = search_state;
    match d {
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
