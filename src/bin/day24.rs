use std::fs::read_to_string;

use regex::Regex;

const BOX_LOW: f64 = 200_000_000_000_000.0;
const BOX_HIGH: f64 = 400_000_000_000_000.0;

struct HailStone {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

fn main() {
    let input = read_to_string("input/day24.txt").unwrap();
    let hail_re = Regex::new(r"^(\d+), *(\d+), *(\d+) *@ *(-?\d+), *(-?\d+), *(-?\d+)$").unwrap();
    let mut hail_stones: Vec<HailStone> = input
        .lines()
        .into_iter()
        .map(|line| {
            let [x, y, z, vx, vy, vz, ..] = *hail_re
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .filter_map(|x| x)
                .map(|m| m.as_str().parse::<isize>().unwrap())
                .collect::<Vec<_>>()
            else {
                panic!("Parse error");
            };
            HailStone { x, y, z, vx, vy, vz }
        })
        .collect();
    hail_stones.sort_by_key(|&HailStone { vx, .. }| vx);
    let mut count = 0;
    for (i, &HailStone { x: x1, y: y1, vx: vx1, vy: vy1, .. }) in hail_stones.iter().enumerate() {
        for &HailStone { x: x2, y: y2, vx: vx2, vy: vy2, .. } in hail_stones.iter().skip(i + 1) {
            let (x1, y1, vx1, vy1, x2, y2, vx2, vy2) = (
                x1 as f64, y1 as f64, vx1 as f64, vy1 as f64, x2 as f64, y2 as f64, vx2 as f64,
                vy2 as f64,
            );
            let x =
                (vx1 * vx2 * (y2 - y1) + vy1 * vx2 * x1 - vy2 * vx1 * x2) / (vy1 * vx2 - vy2 * vx1);
            let y = y1 - x1 * vy1 / vx1
                + (vy1 * vx2 * (y2 - y1) + x1 * vy1 * vy1 * vx2 / vx1 - x2 * vy1 * vy2)
                    / (vx2 * vy1 - vx1 * vy2);
            let t1 = (x - x1) / vx1;
            let t2 = (x - x2) / vx2;
            if t1 > 0.0
                && t2 > 0.0
                && x >= BOX_LOW
                && x <= BOX_HIGH
                && y >= BOX_LOW
                && y <= BOX_HIGH
            {
                count += 1;
            }
        }
    }
    println!("Part 1: {count}");
}

/****

y - y1 = (x - x1) (vy1 / vx1)
(y - y1) / vy1 = (x - x1) / vx1

vx1 (y - y1) = vy1 (x - x1)   // :vx1
vx2 (y - y2) = vy2 (x - x2)   // :vx2

y - y1 = (vy1 / vx1) (x - x1)     /
y - y2 = (vy2 / vx2) (x - x2)    / -

y2 - y1 = (vy1 / vx1)(x - x1) - (vy2 / vx2)(x - x2)
y2 - y1 = (vy1 / vx1) x - x1 vy1 / vx1 - (vy2 / vx2) x + x2 vy2 / vx2
(vy1 / vx1 - vy2 / vx2) x - x1 vy1 / vx1 + x2 vy2 / vx2 + y1 - y2 = 0

x = (x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) / (vy1 / vx1 - vy2 / vx2)
x = (x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) * vx1 * vx2 / (vx2 vy1 - vx1 vy2)

y = y1 + vy1 / vx1 * (x - x1)
y = y1 + vy1 / vx1 * ((x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) * vx1 * vx2 / (vx2 vy1 - vx1 vy2) - x1)
y = y1 - x1 vy1 / vx1 + vy1 vx2 (x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) / (vx2 vy1 - vx1 vy2)

****/
