use std::{
    collections::{BTreeSet, HashSet},
    fs::read_to_string,
};

use regex::Regex;

const BOX_LOW: f64 = 200_000_000_000_000.0;
const BOX_HIGH: f64 = 400_000_000_000_000.0;

#[derive(Clone, Copy)]
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
    let hail_stones: Vec<HailStone> = input
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

    // y - y1 = (x - x1) (vy1 / vx1)
    // (y - y1) / vy1 = (x - x1) / vx1
    //
    // vx1 (y - y1) = vy1 (x - x1)   // :vx1
    // vx2 (y - y2) = vy2 (x - x2)   // :vx2
    //
    // y - y1 = (vy1 / vx1) (x - x1)     /
    // y - y2 = (vy2 / vx2) (x - x2)    / -
    //
    // y2 - y1 = (vy1 / vx1)(x - x1) - (vy2 / vx2)(x - x2)
    // y2 - y1 = (vy1 / vx1) x - x1 vy1 / vx1 - (vy2 / vx2) x + x2 vy2 / vx2
    // (vy1 / vx1 - vy2 / vx2) x - x1 vy1 / vx1 + x2 vy2 / vx2 + y1 - y2 = 0
    //
    // x = (x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) / (vy1 / vx1 - vy2 / vx2)
    // x = (x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) * vx1 * vx2 / (vx2 vy1 - vx1 vy2)
    //
    // y = y1 + vy1 / vx1 * (x - x1)
    // y = y1 + vy1 / vx1 * ((x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) * vx1 * vx2 / (vx2 vy1 - vx1 vy2) - x1)
    // y = y1 - x1 vy1 / vx1 + vy1 vx2 (x1 vy1 / vx1 - x2 vy2 / vx2 + y2 - y1) / (vx2 vy1 - vx1 vy2)

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

    let vrx = rock_speed(
        hail_stones.clone(),
        |&HailStone { vx, .. }| vx,
        |&HailStone { x, .. }| x,
    );
    println!("rock vx = {vrx}"); // 6
    let vry = rock_speed(
        hail_stones.clone(),
        |&HailStone { vy, .. }| vy,
        |&HailStone { y, .. }| y,
    );
    println!("rock vy = {vry}"); // 326
    let vrz = rock_speed(
        hail_stones.clone(),
        |&HailStone { vz, .. }| vz,
        |&HailStone { z, .. }| z,
    );
    println!("rock vz = {vrz}"); // 101

    // (vy2 - vy1) xr + (vx1 - vx2) yr + (y1 - y2) vrx + (x2 - x1) vry = x2 vy2 - y2 vx2 - x1 vy1 + y1 vx1
    // (vy2 - vy1) xr + (vx1 - vx2) yr = x2 vy2 - y2 vx2 - x1 vy1 + y1 vx1 + (y2 - y1) vrx - (x2 - x1) vry
    //
    // a1 = vy2 - vy1
    // b1 = vx1 - vx2
    // c1 = x2 vy2 - x1 vy1 + y1 vx1 - y2 vx2 + (y2 - y1) vrx - (x2 - x1) vry
    //
    // a1 xr + b1 yr = c1
    // a2 xr + b2 yr = c2  // * a1/a2
    // -------------------------------------------
    // a1 xr + b1 yr = c1                     /
    // a1 xr + a1 b2 yr / a2 = c2 a1 / a2    / -
    // -------------------------------------------
    // b1 yr - a1 b2 yr / a2 = c1 - c2 a1 / a2
    // yr ( b1 - a1 b2 / a2 ) = c1 - c2 a1 / a2
    // yr = ( c1 - c2 a1 / a2 ) / ( b1 - a1 b2 / a2 )
    // yr = ( a2 c1 - a1 c2) / ( a2 b1 - a1 b2 )
    // xr = (c1 - b1 yr) / a1
    //
    // vrx = 6, vry = 326, vrz = 101

    let HailStone { x: x1, y: y1, z: z1, vx: vx1, vy: vy1, vz: vz1 } = hail_stones[0];
    let HailStone { x: x2, y: y2, z: z2, vx: vx2, vy: vy2, vz: vz2 } = hail_stones[1];
    let HailStone { x: x3, y: y3, z: z3, vx: vx3, vy: vy3, vz: vz3 } = hail_stones[2];
    let HailStone { x: x4, y: y4, z: z4, vx: vx4, vy: vy4, vz: vz4 } = hail_stones[3];

    let a1 = vy2 - vy1;
    let b1 = vx1 - vx2;
    let c1 = x2 * vy2 - x1 * vy1 + y1 * vx1 - y2 * vx2 + (y2 - y1) * vrx - (x2 - x1) * vry;

    let a2 = vy4 - vy3;
    let b2 = vx3 - vx4;
    let c2 = x4 * vy4 - x3 * vy3 + y3 * vx3 - y4 * vx4 + (y4 - y3) * vrx - (x4 - x3) * vry;

    let yr = (a2 * c1 - a1 * c2) / (a2 * b1 - a1 * b2);
    let xr = (c1 - b1 * yr) / a1;

    let a3 = vz2 - vz1;
    let b3 = vx1 - vx2;
    let c3 = x2 * vz2 - x1 * vz1 + z1 * vx1 - z2 * vx2 + (z2 - z1) * vrx - (x2 - x1) * vrz;

    let a4 = vz4 - vz3;
    let b4 = vx3 - vx4;
    let c4 = x4 * vz4 - x3 * vz3 + z3 * vx3 - z4 * vx4 + (z4 - z3) * vrx - (x4 - x3) * vrz;

    let zr = (a4 * c3 - a3 * c4) / (a4 * b3 - a3 * b4);

    println!("xr, yr, zr: {xr}, {yr}, {zr}");
    println!("Part 2: {}", xr + yr + zr); // 600352360036779
}

fn rock_speed(
    mut hail_stones: Vec<HailStone>,
    speed_fn: impl Fn(&HailStone) -> isize,
    position_fn: impl Fn(&HailStone) -> isize,
) -> isize {
    let mut stone_pairs = Vec::<(HailStone, HailStone)>::new();
    hail_stones.sort_by_key(&speed_fn);
    for i in 0..hail_stones.len() - 1 {
        let stone1 = hail_stones[i];
        let stone2 = hail_stones[i + 1];
        if speed_fn(&stone1) == speed_fn(&stone2) {
            stone_pairs.push((stone1, stone2));
        }
    }
    stone_pairs
        .into_iter()
        .map(|(stone1, stone2)| {
            let x1 = position_fn(&stone1);
            let x2 = position_fn(&stone2);
            let v = speed_fn(&stone1);
            divisors(x1 - x2)
                .into_iter()
                .flat_map(|d| [v + d, v - d])
                .collect::<BTreeSet<isize>>()
        })
        .reduce(|speeds1, speeds2| speeds1.intersection(&speeds2).cloned().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn divisors(n: isize) -> HashSet<isize> {
    let n = n.abs();
    let sqrt = (n as f64).sqrt().ceil() as isize;
    let mut divisors = HashSet::<isize>::new();
    for d in 1..sqrt {
        if n % d == 0 {
            divisors.insert(d);
            divisors.insert(n / d);
        }
    }
    divisors
}
