use priority_queue::PriorityQueue;
use std::fs::read_to_string;

const INFINITY: u32 = 65;

fn main() {
    let input = read_to_string("input/day21.txt").unwrap();
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let mut unvisited: PriorityQueue<(isize, isize), u32> = PriorityQueue::new();
    let mut reachable_in_64_steps = Vec::<(isize, isize)>::new();
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            let plot = grid[y][x];
            if plot == b'#' {
                continue;
            }
            unvisited.push(
                (y as isize, x as isize),
                if plot == b'S' { INFINITY } else { 0 },
            );
        }
    }
    loop {
        let Some(((y, x), priority)) = unvisited.pop() else {
            break;
        };
        if priority == 0 {
            break;
        }
        let visiting_pathlen = INFINITY - priority;
        if visiting_pathlen % 2 == 0 {
            reachable_in_64_steps.push((y, x));
        }
        for (delta_y, delta_x) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let dijkstra_node = (y + delta_y, x + delta_x);
            if let Some((_, neigh_priority)) = unvisited.get(&dijkstra_node) {
                let neigh_pathlen = INFINITY - neigh_priority;
                let this_pathlen = visiting_pathlen + 1;
                if this_pathlen < neigh_pathlen {
                    unvisited.change_priority(&dijkstra_node, INFINITY - this_pathlen);
                }
            }
        }
    }
    println!("Part 1: {}", reachable_in_64_steps.len());
}
