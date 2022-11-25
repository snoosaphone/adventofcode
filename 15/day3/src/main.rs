use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Opening file: input.txt");

    let directions = fs::read_to_string("./input.txt").expect("Could not find input.txt");

    let mut unique_houses: HashSet<[i32; 2]> = HashSet::new();

    let mut santa_pos: [i32; 2] = [0, 0];
    let mut robosanta_pos: [i32; 2] = [0, 0];

    for (i, dir) in directions.chars().enumerate() {
        if i % 2 == 0 {
            match dir {
                '^' => santa_pos[1] += 1,
                '>' => santa_pos[0] += 1,
                'v' => santa_pos[1] -= 1,
                '<' => santa_pos[0] -= 1,
                _ => continue
            }

            if !unique_houses.contains(&santa_pos) {
                unique_houses.insert(santa_pos);
            }
        } else {
            match dir {
                '^' => robosanta_pos[1] += 1,
                '>' => robosanta_pos[0] += 1,
                'v' => robosanta_pos[1] -= 1,
                '<' => robosanta_pos[0] -= 1,
                _ => continue
            }

            if !unique_houses.contains(&robosanta_pos) {
                unique_houses.insert(robosanta_pos);
            }
        }
    }

    let num_houses = unique_houses.len();

    println!("Total unique houses: {num_houses}")
}
