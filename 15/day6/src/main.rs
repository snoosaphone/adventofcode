use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Opening file: input.txt\n");

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    // let mut num_on_lights = 0;
    let mut total_brightness = 0;

    for line in reader.lines() {
        let string = line.unwrap();
        let splits: Vec<_> = string.split(" ").collect();
        let mut instruction_offset = 0;

        if splits.len() == 5 {
            instruction_offset = 1;
        }

        let instruction = splits[instruction_offset];
        let start_set = splits[1 + instruction_offset];
        let end_set = splits[3 + instruction_offset];

        // println!("Instruction: {instruction}");
        // println!("Start: {start_set}");
        // println!("End: {end_set}");

        let start_indices: Vec<_> = start_set.split(",").collect();
        let end_indices: Vec<_> = end_set.split(",").collect();

        let x_start = start_indices[0].parse::<usize>().unwrap();
        let y_start = start_indices[1].parse::<usize>().unwrap();

        let x_end = end_indices[0].parse::<usize>().unwrap();
        let y_end = end_indices[1].parse::<usize>().unwrap();

        for xx in x_start..x_end + 1 {
            for yy in y_start..y_end + 1 {
                // println!("X coord: {xx} Y coord: {yy}");
                match instruction {
                    "off" => if grid[xx][yy] > 0 {
                        grid[xx][yy] -= 1;
                    },
                    "on" => grid[xx][yy] += 1,
                    "toggle" => grid[xx][yy] += 2,
                    _ => continue,
                }
            }
        }
    }

    for xx in 0..1000 {
        for yy in 0..1000 {
            // if grid[xx][yy] == 1 {
            //     num_on_lights += 1;
            // }
            total_brightness += grid[xx][yy];
        }
    }

    println!("");
    // println!("Lights found on: {num_on_lights}");
    println!("Total brightness: {total_brightness}");
}
