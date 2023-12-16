use std::{array, fs::read_to_string};

type Lens<'a> = (&'a [u8], u8);
type BoxArray<'a> = [[Lens<'a>; 6]; 256];

fn main() {
    let input = read_to_string("input/day15.txt").unwrap();
    let init_sequence = input.split(",").map(|s| s.as_bytes());
    let part1 = init_sequence
        .clone()
        .map(|step| hash(step) as usize)
        .sum::<usize>();
    println!("Part 1: {part1}"); // 498538

    let mut box_array: BoxArray = array::from_fn(|_| Default::default());
    for step in init_sequence {
        if let Some(index_of_dash) = step.iter().position(|&b| b == b'-') {
            let label = &step[0..index_of_dash];
            remove(&mut box_array, label);
        } else if let Some(index_of_eq) = step.iter().position(|&b| b == b'=') {
            let label = &step[0..index_of_eq];
            let power = step[index_of_eq + 1] - b'0';
            insert(&mut box_array, label, power);
        } else {
            panic!("Invalid step");
        }
    }
    let mut result = 0;
    for (i, lens_box) in box_array.into_iter().enumerate() {
        for (j, (_, power)) in lens_box.into_iter().enumerate() {
            result += (i + 1) * (j + 1) * power as usize;
        }
    }
    println!("Part 2: {result}"); // 286278
}

fn hash(bytes: &[u8]) -> u8 {
    bytes
        .into_iter()
        .fold(0, |hash, b| hash.wrapping_add(*b).wrapping_mul(17))
}

fn insert<'a, 'b: 'a>(box_array: &mut BoxArray<'a>, label: &'b [u8], power: u8) {
    let hash = hash(&label);
    let lens_box = &mut box_array[hash as usize];
    if let Some((_, stored_power)) = lens_box
        .iter_mut()
        .find(|(stored_label, _)| *stored_label == label)
    {
        *stored_power = power;
    } else {
        let mut inserted = false;
        for lens in lens_box.iter_mut() {
            if lens.0.is_empty() {
                *lens = (label, power);
                inserted = true;
                break;
            }
        }
        if !inserted {
            panic!("Lens box is full!");
        }
    }
}

fn remove(box_array: &mut BoxArray, label: &[u8]) {
    let hash = hash(&label);
    let lens_box = &mut box_array[hash as usize];
    if let Some(pos) = lens_box
        .iter()
        .position(|&(label_in_box, _)| label_in_box == label)
    {
        for i in pos..lens_box.len() - 1 {
            lens_box[i] = lens_box[i + 1];
        }
        lens_box[lens_box.len() - 1] = (&[], 0);
    }
}
