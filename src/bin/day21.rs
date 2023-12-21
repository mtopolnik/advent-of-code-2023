use priority_queue::PriorityQueue;
use std::{fs::read_to_string, iter::repeat};

const N: usize = 202_300;
const MAX_STEPS: usize = N * 131 + 65;
const INFINITY: usize = MAX_STEPS + 1;

fn main() {
    println!("N = {N}");
    println!(
        "Predicted reachable count: {}",
        3_917 + 15_550 * N + 15_453 * N * N // Part 2, for N = 202_300: 632,421,652,138,917
    );
    let input = read_to_string("input/day21.txt").unwrap();
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let mut unvisited: PriorityQueue<((isize, isize), (isize, isize)), usize> =
        PriorityQueue::new();
    let mut reachable = Vec::<((usize, usize), (usize, usize))>::new();
    let size = grid.len();
    let super_size = super_size(size);
    for super_y in 0..super_size {
        for super_x in 0..super_size {
            for y in 0..size {
                for x in 0..size {
                    let plot = grid[y][x];
                    if plot == b'#' {
                        continue;
                    }
                    unvisited.push(
                        (
                            (super_y as isize, y as isize),
                            (super_x as isize, x as isize),
                        ),
                        if (plot, super_x, super_y) == (b'S', super_size / 2, super_size / 2) {
                            INFINITY
                        } else {
                            0
                        },
                    );
                }
            }
        }
    }
    loop {
        let Some((((super_y, y), (super_x, x)), priority)) = unvisited.pop() else {
            break;
        };
        if priority == 0 {
            break;
        }
        let visiting_pathlen = INFINITY - priority;
        if visiting_pathlen % 2 != INFINITY % 2 {
            reachable.push((
                (super_y as usize, y as usize),
                (super_x as usize, x as usize),
            ));
        }
        for (delta_y, delta_x) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let dijkstra_node = (
                add_delta(super_y, y, size, delta_y),
                add_delta(super_x, x, size, delta_x),
            );
            if let Some((_, neigh_priority)) = unvisited.get(&dijkstra_node) {
                let neigh_pathlen = INFINITY - neigh_priority;
                let this_pathlen = visiting_pathlen + 1;
                if this_pathlen < neigh_pathlen {
                    unvisited.change_priority(&dijkstra_node, INFINITY - this_pathlen);
                }
            }
        }
    }
    println!("Actual reachable count: {}", reachable.len()); // Part 1, for MAX_STEPS = 64: 3820

    // debug_print(grid, reachable);
}

fn add_delta(mut super_v: isize, mut v: isize, len: usize, delta: isize) -> (isize, isize) {
    let len = len as isize;
    v += delta;
    if v < 0 {
        v = len - 1;
        super_v -= 1;
    } else if v == len {
        v = 0;
        super_v += 1;
    }
    (super_v, v)
}

fn super_size(size: usize) -> usize {
    (2 * INFINITY as usize) / size + 2
}

fn debug_print(grid: Vec<&[u8]>, reachable: Vec<((usize, usize), (usize, usize))>) {
    let size = grid.len();
    let super_size = super_size(size);
    let grid1: Vec<Vec<u8>> = grid
        .into_iter()
        .map(|row| {
            repeat(row.to_vec())
                .take(super_size)
                .flat_map(|x| x)
                .collect::<Vec<_>>()
        })
        .collect();
    let mut traveled_grid: Vec<Vec<u8>> = repeat(grid1).take(super_size).flat_map(|x| x).collect();
    for ((super_y, y), (super_x, x)) in reachable {
        traveled_grid[size * super_y + y][size * super_x + x] = b'O';
    }
    for row in traveled_grid {
        println!("{}", String::from_utf8_lossy(&row));
    }
}
